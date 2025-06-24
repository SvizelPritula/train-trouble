import { teams, type Team } from "./channels";

const teamKey = "team";

export function loadTeam(): Team | null {
    try {
        let id = localStorage.getItem(teamKey);

        for (let team of teams) {
            if (team.id == id)
                return team.id;
        }
    } catch { }

    return null;
}

export function storeTeam(team: Team) {
    try {
        localStorage.setItem(teamKey, team);
    } catch { }
}
