document.addEventListener("DOMContentLoaded", function() {
    const svg = document.getElementById("blines");

    function drawLine(x1, y1, x2, y2) {
        const line = document.createElementNS("http://www.w3.org/2000/svg", "line");
        line.setAttribute("x1", x1);
        line.setAttribute("y1", y1);
        line.setAttribute("x2", x2);
        line.setAttribute("y2", y2);
        line.setAttribute("stroke", "black");
        line.setAttribute("stroke-width", "2");
        svg.appendChild(line);
    }

    function connectTeams(x1, y1, x2, y2, x3, y3, midX) {
        // draw line for the first team
        drawLine(x1, y1, midX, y1);
        // draw line for the second team
        drawLine(x2, y2, midX, y2);
        // draw line connecting the two teams
        drawLine(midX, y1, midX, y2);
        // draw line connecting the winner
        drawLine(midX, y3, x3, y3);
    }

    const bracket = this.getElementById("bracket");

    // iterate over each bracket
    const matches = bracket.getElementsByClassName("bcolumn");

    for (let i = 0; i < matches.length - 1; i++) {
        const match = matches[i];
        const nextMatch = matches[i + 1];

        const teamContainers = match.getElementsByClassName("bteamcontainer");
        const nextTeamContainers = nextMatch.getElementsByClassName("bteamcontainer");
        
        for (let j = 0; j < teamContainers.length; j += 2) {
            const team1 = teamContainers[j].getElementsByClassName("bteam")[0];
            const team2 = teamContainers[j + 1].getElementsByClassName("bteam")[0];
            const nextTeam = nextTeamContainers[j / 2].getElementsByClassName("bteam")[0];

            if (!team1 || !team2 || !nextTeam) {
                continue;
            }
            
            const team1Rect = team1.getBoundingClientRect();
            const team2Rect = team2.getBoundingClientRect();
            const nextTeamRect = nextTeam.getBoundingClientRect();

            const teams12_center = (team1Rect.left + team1Rect.width / 2 + team2Rect.left + team2Rect.width / 2) / 2;
            const team3_center = nextTeamRect.left + nextTeamRect.width / 2;
            const midX = (teams12_center + team3_center) / 2;

            connectTeams(
                team1Rect.right,
                team1Rect.top + team1Rect.height / 2,
                team2Rect.right,
                team2Rect.top + team2Rect.height / 2,
                nextTeamRect.left,
                nextTeamRect.top + nextTeamRect.height / 2,
                midX
            );
        }
    }
});
