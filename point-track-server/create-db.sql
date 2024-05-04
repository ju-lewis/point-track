
/* Yacht Club Information */
CREATE TABLE IF NOT EXISTS yachtClub (
	yachtClubId INTEGER PRIMARY KEY AUTOINCREMENT,
	name VARCHAR(255) NOT NULL
);


/* Login credentials */
CREATE TABLE IF NOT EXISTS account (
	yachtClubId INTEGER NOT NULL,
	username VARCHAR(100) UNIQUE NOT NULL,
	password VARCHAR(64) NOT NULL,

	FOREIGN KEY(yachtClubId) REFERENCES yachtClub(yachtClubId)
);

CREATE TABLE IF NOT EXISTS account_session (
	yachtClubId INTEGER NOT NULL,
	token VARCHAR(20),

	FOREIGN KEY(yachtClubId) REFERENCES yachtClub(yachtClubId)
);

/* Race Table */
CREATE TABLE IF NOT EXISTS race (
	raceId INTEGER PRIMARY KEY AUTOINCREMENT,
	raceDate DATETIME NOT NULL,
	yachtClubId INTEGER NOT NULL,

	FOREIGN KEY(yachtClubId) REFERENCES yachtClub(yachtClubId)
);

/* Store boats */
CREATE TABLE IF NOT EXISTS boat (
	boatId INTEGER PRIMARY KEY AUTOINCREMENT,
	compNumber INTEGER NOT NULL,
	name VARCHAR(100) NOT NULL,
	skipper VARCHAR(100) NOT NULL,
	navigator VARCHAR(100) NOT NULL,
	yachtClubId INTEGER NOT NULL,

	FOREIGN KEY(yachtClubId) REFERENCES yachtClub(yachtClubId)
);

CREATE TABLE IF NOT EXISTS boatRace (
	boatId INTEGER NOT NULL,
	raceId INTEGER NOT NULL,
	nominatedSpeed TINYINTEGER NOT NULL,
	-- Note: The following fields can all be null since the table also stores registered but yet to compete boats.
	penalty FLOAT,
	pointsLost FLOAT,
	handicap FLOAT,
	totalPoints FLOAT,
	notes VARCHAR(100),

	PRIMARY KEY (boatId, raceId),
	FOREIGN KEY(boatId) REFERENCES boat(boatId),
	FOREIGN KEY(raceId) REFERENCES race(raceId)
);

/* Stores the pre-recorded points used in the races */
CREATE TABLE IF NOT EXISTS coursePoint (
	pointId INTEGER PRIMARY KEY AUTOINCREMENT,
	name VARCHAR(80) NOT NULL,
	latitude INTEGER NOT NULL, 
	longitude INTEGER NOT NULL
	-- Note: The lat and lon fields are integers using dddmmsss format
	-- passingSide BOOLEAN  -- 0 = port, 1 = starboard
);

/* The intended positions/times for the boat based on nom. speed */
CREATE TABLE IF NOT EXISTS boatPredictedPositions (
	pointId INTEGER NOT NULL,
	boatId INTEGER NOT NULL,
	raceId INTEGER NOT NULL,
	elapsedTime DATETIME NOT NULL,

	FOREIGN KEY(pointId) REFERENCES coursePoint(pointId),
	FOREIGN KEY(boatId) REFERENCES race(boatId),
	FOREIGN KEY(raceId) REFERENCES race(raceId)
);

CREATE TABLE IF NOT EXISTS boatRacePositionUpdate (
	pointId INTEGER NOT NULL,
	boatId INTEGER NOT NULL,
	raceId INTEGER NOT NULL,

	latitude FLOAT NOT NULL,
	longitude FLOAT NOT NULL,
	elapsedTime DATETIME NOT NULL,


	FOREIGN KEY(pointId) REFERENCES coursePoint(pointId),
	FOREIGN KEY(boatId) REFERENCES race(boatId),
	FOREIGN KEY(raceId) REFERENCES race(raceId)
);
