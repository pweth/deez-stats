<?php
session_start();
//If user has not authenicated, return to homepage
if (!isset($_SESSION["user"]) || !isset($_SESSION["data"])) {
	header("Location: https://deezstats.com");
	exit();
}

$token = $_SESSION["user"];

//Gather basic information about the user and their account
$user = json_decode(file_get_contents("https://api.deezer.com/user/me?access_token=" . $token));
$data = [
	"name" => htmlspecialchars($user->name),
	"picture" => $user->picture_medium
];
?>
<!doctype html>
<html>
<head>
	<meta charset="utf-8">
	<title>Deez Stats</title>
	<meta name="csrf" data-code="<?php echo $_SESSION["data"]; ?>">
	<meta name="robots" content="noindex, nofollow">
	<meta name="viewport" content="width=device-width, initial-scale=1">
	<meta name="theme-color" content="#479ECC">
	<link rel="icon" href="favicon.png">
	<link rel="stylesheet" href="css/display.css">
	<script src="js/jquery.js"></script>
</head>
<body>
	<div id="loader">
		<div>
			<h1>
				Hang tight <span class="name"><?php echo $data["name"]; ?>.</span>
				<br>
				I am grabbing your data...
			</h1>
			<img draggable="false" src="img/loading.gif">
		</div>
	</div>
	<content>
		<div class="top">
			<div class="box1 ani"></div>
			<div class="box2 ani"></div>
			<div class="onehundred">
				<img draggable="false" class="me" src="<?php echo $data["picture"]; ?>">
				<div class="container tagline">
					<h1>
						Hey <?php echo $data["name"]; ?>.
						<br>
						Check out <div class="deez ani">Deez Stats.</div>
					</h1>
				</div>
			</div>
		</div>
		<div class="page fax">
			<div class="container">
				<div class="fax-box mar ani" id="playlistNum">
					<h1>
						<img draggable="false" src="img/playlist.png">
						<span class="DATA count-up ani" data-count=""></span> playlists
					</h1>
				</div>
				<div class="fax-box mal ani" id="recentNum">
					<h1>	
						<span class="DATA count-up ani" data-count=""></span> recent songs
						<img draggable="false" src="img/recent.png">
					</h1>
				</div>
				<div class="fax-box mar ani" id="favNum">
					<h1>
						<img draggable="false" src="img/heart.png">
						<span class="DATA count-up ani" data-count=""></span> favourite tracks
					</h1>
				</div>
				<div class="fax-box mal ani" id="explicitNum">
					<h1>
						<span class="DATA count-up ani" data-count=""></span> with explicit lyrics
						<img draggable="false" src="img/explicit.png">
					</h1>
				</div>
				<div class="fax-box mar ani" id="duration">
					<h1>
						<img draggable="false" src="img/duration.png">
						Average length: <span class="MIN count-up ani" data-count=""></span>m <span class="SEC count-up ani" data-count=""></span>s
					</h1>
				</div>
			</div>
		</div>
		<div class="page top-songs">
			<div class="container">
				<h1>Recently you've been <div class="deez ani been-liking">liking</div>...</h1>
				<div></div>
			</div>
		</div>
		<div class="page top-artists">
			<div class="container">
				<h1><span class="colw">/</span>Top Artists<span class="colw">/</span></h1>
				<div></div>
			</div>
		</div>
		<div class="page unique">
			<div class="container">
				<h1 class="fadein big-title ani">
					<div class="deez-unique">
						Deez Score
					</div>
					<div class="sub-title">
						(higher number = more mainstream)
					</div>
				</h1>
				<div class="rank-box" id="recentRank">
					<div>
						RECENTLY
						<h1 class="DATA ani count-up" data-count=""></h1>
					</div>
				</div>
				<div class="rank-box" id="allTimeRank">
					<div>
						ALL TIME
						<h1 class="DATA ani count-up" data-count=""></h1>
					</div>
				</div>
			</div>
		</div>
		<div class="page share">
			<div class="container">
				<h1>Share Deez Stats and compare with your friends.</h1>
				<a href="https://twitter.com/intent/tweet?url=deezstats.com" target="_blank">
					<img draggable="false" src="img/twitter.png">
				</a>
				<a href="https://www.facebook.com/sharer/sharer.php?u=deezstats.com" target="_blank">
					<img draggable="false" src="img/facebook.png">
				</a>
				<a href="mailto:?subject=Check%20out%20Deez%20Stats&body=https://deezstats.com" target="_blank">
					<img draggable="false" src="img/email.png">
				</a>
			</div>
		</div>
	</content>
	<script src="js/script.js" type="module"></script>
</body>
</html>