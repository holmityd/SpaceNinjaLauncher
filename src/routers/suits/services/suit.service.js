function getSuitLevelXP() {
    const start = 1000;
    const res = [];
    for (let i = 0; i < 30; i++) {
        res.push(i * 2000 + start + (res[i - 1] ? res[i - 1] : 0));
    }
    return res;
}

export const suitLvlXP = getSuitLevelXP();

export function getSuitLvlByXP(XP) {
    let res = 30;
    for (let i = 0; i < suitLvlXP.length; i++) {
        if (XP < suitLvlXP[i]) {
            res = i;
            break;
        }
    }
    return res;
}
