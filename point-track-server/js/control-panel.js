
const UPDATE_PERIOD = 60; // Seconds

async function queryResults() {
	let response = await (await fetch("/poll-results")).text();
	console.log("A");

	// Now update table body
	document.getElementById("results").innerHTML = response;
}

async function queryNTRIP() {
	
}

async function startRace() {

	// Update button
	let startButton = document.getElementById("start-btn");
	startButton.innerHTML = "Stop Race";
	startButton.setAttribute("onclick", "stopRace()");


	// Now loop while requesting result updates
	setInterval(queryResults, UPDATE_PERIOD * 1000);
}


function stopRace() {
	// Update button
	let startButton = document.getElementById("start-btn");
	startButton.innerHTML = "Start Race";
	startButton.setAttribute("onclick", "startRace()");

	clearInterval(queryResults);
}


async function registerBoat() {

	const compNum = parseInt(document.getElementById("comp-num").value);
	const pointElements = document.getElementById("race-points").children;
	
	let raceDate = document.getElementById("race-date").value;

	// Store all of the point IDs
	let pointList = new Array();
	for(let i=0; i<pointElements.length; i++) {

		let pointId = parseInt(pointElements[i].querySelector(".race-point").value);
		let pointSide = parseInt(pointElements[i].querySelector(".race-point-side").value);
		let pointTime = pointElements[i].querySelector(".passing-time").value;

		console.log(raceDate + " " + pointTime);

		let pointDateTime = (new Date(raceDate + " " + pointTime)).getTime();

		// Filter out NaNs
		pointId = isNaN(pointId) ? -1 : pointId;
		

		pointList.push({"id": pointId, "side": pointSide, "time": pointDateTime});
	}

	console.log({"boat": compNum, "points": pointList});

	// Register the boat on the server
	const response = await fetch("/register-boat", {
		method: "post",
		headers: {
			"content-type": "application/json"
		},
		body: JSON.stringify({"boat": compNum, "points": pointList})
	});
	const responseVal = await response.json();
}

