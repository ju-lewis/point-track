
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

function parseTimeStr(timeStr) {
	const elements = timeStr.split(":");

	// Convert the HH:MM:SS time to integer seconds
	return parseInt(elements[0])*3600 + parseInt(elements[1])*60 + parseInt(elements[2]);
}


async function registerBoat() {

	const compNum = parseInt(document.getElementById("comp-num").value);
	const pointElements = document.getElementById("race-points").children;
	let raceDate = new Date(document.getElementById("race-date").value);
	let raceUnixTime = raceDate.getTime()*1000;

	let nomSpeed = parseInt(document.getElementById("nom-speed").value);

	// Store all of the point IDs
	let pointList = new Array();
	for(let i=0; i<pointElements.length; i++) {

		let pointId = parseInt(pointElements[i].querySelector(".race-point").value);
		let pointSide = parseInt(pointElements[i].querySelector(".race-point-side").value);
		let pointTime = parseTimeStr(pointElements[i].querySelector(".passing-time").value);


		console.log(raceDate + " " + pointTime);

		let pointDateTime = raceDate.getTime()*1000 + pointTime;

		// Filter out NaNs
		pointId = isNaN(pointId) ? -1 : pointId;
		

		pointList.push({"id": pointId, "side": pointSide, "time": pointDateTime});
	}

	console.log({"boat": compNum, "race_date": raceUnixTime, "nom_speed": nomSpeed, "points": pointList});

	// Register the boat on the server
	const response = await fetch("/race-register-boat", {
		method: "POST",
		headers: {
			"content-type": "application/json"
		},
		body: JSON.stringify({"boat": compNum, "race_date": raceUnixTime, "nom_speed": nomSpeed, "points": pointList})
	});
	
	const status = response.status;
	if(status == 422) {
		// There was a parsing error, notify user
	} else if (status == 404) {
		// The boat has not been registered in the system yet
		alert(`You must first register boat ${compNum} in the system before they can join a race.`);
	} else if (status != 200) {
		// There was another kind of error
	}

	// We know registration was successful, re-query registered boat list
	getRegisteredBoats();
}

async function getRegisteredBoats() {
	// Multiply by 1000 to convert from milliseconds since Epoch to Unix time
	const raceUnixDate = (new Date(document.getElementById("race-date").value)).getTime()*1000;


	const res = await fetch(`/get-registered-boats?date=${raceUnixDate}`);

	if(res.status != 200) {
		// Show error modal, couldn't retrieve registered boats 
	}
	const boats = await res.json();

	console.log("Registered boats:");
	console.log(boats);
}

