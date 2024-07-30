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

    function connectTeams(x1, y1, x2, y2, midX, lineToWinner) {
        // draw line for the participant
        drawLine(x1, y1, midX, y1);
        // draw line connecting the two teams
        drawLine(midX, y1, midX, y2);
        // draw line connecting the winner
        if (lineToWinner) {
            drawLine(midX, y2, x2, y2);
        }
    }

    const bracket = this.getElementById("bracket");

    // iterate over each bracket
    const matches = bracket.getElementsByClassName("bcolumn");

    for (let i = 0; i < matches.length - 1; i++) {
        const match = matches[i];
        const nextMatch = matches[i + 1];

        const teamContainers = match.getElementsByClassName("bteamcontainer");
        const nextTeamContainers = nextMatch.getElementsByClassName("bteamcontainer");
        
        for (let j = 0; j < teamContainers.length; j++) {
            const currentTeam = teamContainers[j].getElementsByClassName("bteam")[0];
            const winningTeam = nextTeamContainers[Math.floor(j / 2)].getElementsByClassName("bteam")[0];

            if (!currentTeam || !winningTeam) {
                continue;
            }
            
            const currentTeamRect = currentTeam.getBoundingClientRect();
            const winningTeamRect = winningTeam.getBoundingClientRect();

            const currentTeamCenter = currentTeamRect.left + currentTeamRect.width / 2;
            const winningTeamCenter = winningTeamRect.left + winningTeamRect.width / 2;
            const midX = (currentTeamCenter + winningTeamCenter) / 2;

            let lineToWinner = (j % 2 === 0);
            if (teamContainers[j - 1] && !(teamContainers[j - 1].getElementsByClassName("bteam")[0])) {
                lineToWinner = true;
            }

            connectTeams(
                currentTeamRect.right,
                currentTeamRect.top + currentTeamRect.height / 2,
                winningTeamRect.left,
                winningTeamRect.top + winningTeamRect.height / 2,
                midX,
                true
            );
        }
    }
});
