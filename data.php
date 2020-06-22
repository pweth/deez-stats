<?php
session_start();
//If user or request is not authenicated, return nothing
if (!isset($_SESSION["data"]) || !isset($_GET["code"]) || $_SESSION["data"] !== $_GET["code"]) {
	echo "{}";
	exit();
}
//Only allow one request
unset($_SESSION["data"]);

//Function to gather data from a specified API endpoint until there is no "next" url in the response
function gather($next) {
	$data = array();
	do {
		$response = json_decode(file_get_contents($next));
		$data = array_merge($data, $response->data);
		if (isset($response->next)) {
			$next = $response->next;
		}
	} while (isset($response->next));
	return $data;
}

$token = $_SESSION["user"];

//Gather basic information about the user and their account
$user = json_decode(file_get_contents("https://api.deezer.com/user/me?access_token=" . $token));
$id = $user->id;

//Gather recent history of tracks listened to
$history = gather("https://api.deezer.com/user/" . $id . "/history?access_token=" . $token);

//Gather user's playlists
$playlists = gather("https://api.deezer.com/user/" . $id . "/playlists?access_token=" . $token);

//Gather user's loved tracks
$lovedTracks = gather("https://api.deezer.com/user/" . $id . "/tracks?access_token=" . $token);

//Explicit count
$explicitCount = 0;
//Duration count
$durCount = 0;
//Calculate 'uniqueness' of music taste by adding up Deezer ranks
$totalPosition = 0;
//Artist count
$artistCount = array();
//Artist data
$artistData = array();
//Loop through the user's loved tracks
for ($i = 0; $i < sizeOf($lovedTracks); $i++) {
	//If track has explicit lyrics, increase counter
	if ($lovedTracks[$i]->explicit_lyrics) {
		$explicitCount++;
	}
	//Add duration to counter
	$durCount += $lovedTracks[$i]->duration;
	//Add Deezer rank to total
	$totalPosition += $lovedTracks[$i]->rank;
	//Create or increment artist counter
	$tempId = $lovedTracks[$i]->artist->id;
	if (isset($artistCount[$tempId])) {
		$artistCount[$tempId] += 1;
	} else {
		$artistCount[$tempId] = 1;
	}
	//Store artist data for later
	if (!isset($artistData[$tempId])) {
		$artistData[$tempId] = array(
			$lovedTracks[$i]->artist->name,
			$lovedTracks[$i]->artist->picture_medium
		);
	}
}

//Sort artist count from high to low, retaining key-value pairs
arsort($artistCount);
//Take the top ten artists (if possible)
$topArtists = array();
if (sizeOf($artistCount) > 10) {
	for ($i = 0; $i < 10; $i++) {
		array_push($topArtists, $artistData[array_keys($artistCount)[$i]]);
	}
} else {
	for ($i = 0; $i < sizeOf($artistCount); $i++) {
		array_push($topArtists, $artistData[array_keys($artistCount)[$i]]);
	}
}

//Average duration
$avDur = round($durCount / sizeOf($lovedTracks));
$avDurSecs = $avDur % 60;
$avDurMins = ($avDur - $avDurSecs) / 60;

//Divide the total of positions by the number of tracks
$uniqueness = round($totalPosition / sizeOf($lovedTracks));

//Calculate top tracks from recent listening
$recentPosition = 0;
$recentTracks = array();
$trackData = array();
//Deezer rank count from recent listening
for ($i = 0; $i < sizeOf($history); $i++) {
	//Add track ranking to total recent rank count
	$recentPosition += $history[$i]->rank;
	$id = $history[$i]->id;
	//Store track data for later
	if (!isset($trackData[$id])) {
		$trackData[$id] = array(
			$history[$i]->title,
			$history[$i]->artist->name,
			$history[$i]->album->cover_medium
		);
	}
	//Increment or setup track counter
	if (isset($recentTracks[$id])) {
		$recentTracks[$id] += 1;
	} else {
		$recentTracks[$id] = 1;
	}
}

//Sort track count from high to low, retaining key-value pairs
arsort($recentTracks);
//Select top five recent tracks
$topTracks = array();
if (sizeOf($recentTracks) > 5) {
	for ($i = 0; $i < 5; $i++) {
		array_push($topTracks, $trackData[array_keys($recentTracks)[$i]]);
	}
} else {
	for ($i = 0; $i < sizeOf($recentTracks); $i++) {
		array_push($topTracks, $trackData[array_keys($recentTracks)[$i]]);
	}
}

//Divide the total of positions by the number of tracks
$recentUniqueness = round($recentPosition / sizeOf($history));

//Build up a JSON object that can be returned
$object = [];
$object["playlistNum"] = sizeOf($playlists);
$object["favNum"] = sizeOf($lovedTracks);
$object["favUnique"] = $uniqueness;
$object["explicitNum"] = $explicitCount;
$object["avDurMins"] = $avDurMins;
$object["avDurSecs"] = $avDurSecs;
$object["topArtists"] = $topArtists;
$object["recentNum"] = sizeOf($history);
$object["recentUnique"] = $recentUniqueness;
$object["topTracks"] = $topTracks;
echo json_encode($object);