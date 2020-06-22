<?php
$app_id     = "<REDACTED>";
$app_secret = "<REDACTED>";
$my_url     = "https://deezstats.com/auth";
 
session_start();
if (isset($_SESSION["user"]) && isset($_SESSION["data"])) {
	header("Location: https://deezstats.com/display");
}

$code = $_REQUEST["code"];
 
if (!isset($code) || empty($code)){
	$_SESSION["state"] = md5(uniqid(rand(), true));
	$dialog_url = "https://connect.deezer.com/oauth/auth.php?app_id=" . $app_id . "&redirect_uri=" . urlencode($my_url) . "&perms=basic_access,email,listening_history&state=" . $_SESSION["state"];
	header("Location: " . $dialog_url);
	exit();
}

if ($_REQUEST["state"] == $_SESSION["state"]) {
	$response = file_get_contents("https://connect.deezer.com/oauth/access_token.php?app_id=" . $app_id . "&secret=" . $app_secret . "&code=" . $code);
	$params = null;
	parse_str($response, $params);
	$_SESSION["user"] = $params["access_token"];
	$_SESSION["data"] = md5(uniqid(rand(), true));
	header("Location: https://deezstats.com/display");
	exit();
} else {
	header("Location: https://deezstats.com");
	exit();
}