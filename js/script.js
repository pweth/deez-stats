//Countup.js script
import { CountUp } from "/js/countup.js";

//Global variables
let deezStats = {};
let deezFunctions = {};

deezFunctions.setup = (data) => {
	console.log("Statistics loaded successfully!");
	//Fill in appropriate data
	$("#recentRank .DATA").attr("data-count", data.recentUnique);
	$("#allTimeRank .DATA").attr("data-count", data.favUnique);
	$("#playlistNum .DATA").attr("data-count", data.playlistNum);
	$("#favNum .DATA").attr("data-count", data.favNum);
	$("#recentNum .DATA").attr("data-count", data.recentNum);
	$("#explicitNum .DATA").attr("data-count", data.explicitNum);
	$("#duration .MIN").attr("data-count", data.avDurMins);
	$("#duration .SEC").attr("data-count", data.avDurSecs);
	for (let i = 0; i < data.topArtists.length; i++) {
		let temp = "<div class=\"artist ani\">";
		temp += "<img draggable=\"false\" src=\"" + data.topArtists[i][1] + "\">";
		temp += "<div><br>" + data.topArtists[i][0];
		temp += "</div></div>";
		$(".top-artists .container > div").append(temp);
	}
	for (let i = 0; i < data.topTracks.length; i++) {
		let temp = "<div class=\"song ani\">";
		temp += "<img draggable=\"false\" src=\"" + data.topTracks[i][2] + "\">";
		temp += "<div><b>" + data.topTracks[i][0] + "</b><br>";
		temp += data.topTracks[i][1] + "</div></div>";
		$(".top-songs .container > div").append(temp);
	}
	//Animate loader up and off the screen
	$("#loader").animate({top: "-100%"}, 250, () => {
		$("#loader").finish().hide();
		//Initiate first animations
		$("content").trigger("scroll");
	});
};

//To enable scroll-based animations
deezFunctions.scrolling = () => {
	let windowHeight = $(window).height();
	let windowTopPosition = $(window).scrollTop();
	let windowBottomPosition = windowTopPosition + windowHeight;
	$(".ani").each(function (i, obj) {
		let elementHeight = $(this).outerHeight();
		let elementTopPosition = $(this).offset().top;
		let elementBottomPosition = elementTopPosition + elementHeight;
		//Check to see if this current element is within viewport
		if ((elementBottomPosition >= windowTopPosition) &&
			(elementTopPosition <= windowBottomPosition)) {
			//Enable counting up animations
			if ($(this).hasClass("count-up") && !($(this).hasClass("seen"))) {
				let cu = new CountUp(this, parseInt($(this).attr("data-count")));
				cu.start();
			}
			$(this).addClass("seen");
		} else {
			$(this).removeClass("seen");
		}
	});
};
//Attach relevant event listeners
$("content").on("scroll", deezFunctions.scrolling);
window.onresize = deezFunctions.scrolling;

$(document).ready(() => {
	console.log("Loading statistics...");
	//CSRF
	let csrf = $("meta[name=csrf]").attr("data-code");
	//Request the rest of the data asynchronously
	fetch("/data?code=" + encodeURI(csrf))
		.then((response) => response.json())
		.then((data) => deezFunctions.setup(data));
});