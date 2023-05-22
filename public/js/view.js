// Countup.js script
import { CountUp } from "/js/countup.js";

const deezStats = {};

deezStats.setup = data => {
    // Fill in appropriate data
    $("img.me").attr("src", data.user.picture_medium);
    $("span.me").html(data.user.name);
	$("#recentRank .DATA").attr("data-count", data.numbers.uniqueness.recent);
	$("#allTimeRank .DATA").attr("data-count", data.numbers.uniqueness.loved);
	$("#playlistNum .DATA").attr("data-count", data.numbers.playlists);
	$("#favNum .DATA").attr("data-count", data.numbers.loved_tracks);
	$("#recentNum .DATA").attr("data-count", data.numbers.recent_tracks);
	$("#explicitNum .DATA").attr("data-count", data.numbers.explicit);
	$("#duration .MIN").attr("data-count", data.numbers.average.minutes);
	$("#duration .SEC").attr("data-count", data.numbers.average.seconds);
    for (let artist of data.artists) {
        $(".top-artists .container > div").append(`<div class="artist ani">
            <img draggable="false" src="${artist.picture}">
            <div>
                <br>${artist.name}
            </div>
        </div>`);
    }
    for (let track of data.tracks) {
        $(".top-songs .container > div").append(`<div class="song ani">
            <img draggable="false" src="${track.picture}">
            <div>
                <br><b>${track.title}</b>
                <br>${track.name}
            </div>
        </div>`);
    }
    // Animate loader up and off the screen
	$("#loader").animate({top: "-100%"}, 250, () => {
		$("#loader").finish().hide();
		// Initiate first animations
		$("content").trigger("scroll");
	});
};

// To enable scroll-based animations
deezStats.scroll = () => {
	let windowHeight = $(window).height();
	let windowTopPosition = $(window).scrollTop();
	let windowBottomPosition = windowTopPosition + windowHeight;
	$(".ani").each(function (i, obj) {
		let elementHeight = $(this).outerHeight();
		let elementTopPosition = $(this).offset().top;
		let elementBottomPosition = elementTopPosition + elementHeight;
		// Check to see if this current element is within viewport
		if ((elementBottomPosition >= windowTopPosition) &&
			(elementTopPosition <= windowBottomPosition)) {
			// Enable counting up animations
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

$(document).ready(() => {
    // Attach relevant event listeners
    $("content").on("scroll", deezStats.scroll);
    window.onresize = deezStats.scroll;
    // First extract data from the URI
    const search = new URL(document.URL).search;
    if (search.length > 2) {
        const uuid = search.substring(1);
        // Check that data hasn't already been generated
        fetch(`/data/${uuid}`).then(response => response.json()).then(response => {
            if (response.success) {
                // Setup the page
                deezStats.setup(response);
            } else {
                // Try to generate the data
                fetch(`/generate/${uuid}`).then(response => response.json()).then(response => {
                    if (response.success) {
                        // Re-fetch data and setup the page
                        fetch(`/data/${uuid}`).then(response => response.json()).then(response => deezStats.setup(response));
                    } else {
                        // Show error message
                        $("#error").css("display", "flex");
                    }
                });
            }
        });
    } else {
        // Show error message
        $("#error").css("display", "flex");
    }
});