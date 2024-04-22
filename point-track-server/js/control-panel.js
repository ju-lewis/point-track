
var raceRunning = false;

const UPDATE_PERIOD = 60; // Seconds

function delay(millis) {
	return new Promise(resolve => {
		setTimeout(() => { resolve('') }, millis);
	});
}

async function startRace() {

	// Update button
	let startButton = document.getElementById("start-btn");
	startButton.innerHTML = "Stop Race";
	startButton.setAttribute("onclick", "stopRace()");

	raceRunning = true;

	// Now loop while requesting result updates
	while(raceRunning) {
		let response = await (await fetch("/poll-results")).text();
		//console.log(response);

		// Now update table body
		document.getElementById("results").innerHTML = response;
		await delay(UPDATE_PERIOD);
	}

}

function stopRace() {
	// Update button
	let startButton = document.getElementById("start-btn");
	startButton.innerHTML = "Start Race";
	startButton.setAttribute("onclick", "startRace()");
}
