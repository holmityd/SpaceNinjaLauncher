import { get } from "svelte/store";
import { userStore } from "../store/User.store";

const LOCAL_DATA = import.meta.env.VITE_LOCAL_DATA;

const serverUrl = "http://localhost:53426";

export async function getUsers() {
    let apiUrl = `${serverUrl}/accounts`;

    if (LOCAL_DATA) {
        apiUrl = "api/users.json";
    }

    const response = await fetch(apiUrl);
    const data = await response.json();
    return data;
}

/**
 * @param {{id: string, email: string, display_name: string}} user - The user object.
 */
export async function fetchUserData(user) {
    let inventoryUrl = `${serverUrl}/inventory/${user.id}`;

    if (LOCAL_DATA) {
        inventoryUrl = "api/user.json";
    }

    const response = await fetch(inventoryUrl);
    const inventory = await response.json();
    userStore.set({ ...user, inventory });
}

/**
 * @param {Array<{ItemType: string, ItemCount: number, UpgradeFingerprint: string, _id: {$oid: string}}>} postItems - The items to be removed.
 */
export async function removeMods(postItems) {
    doModsRequestAndChangeInStore("remove", postItems);
}

/**
 * @param {Array<{ItemType: string, ItemCount?: number, UpgradeFingerprint?: string}>} postItems - The items to be added.
 */
export async function addMods(postItems) {
    doModsRequestAndChangeInStore("add", postItems);
}

/**
 * @param {{_id: {$oid: string}, UpgradeFingerprint:string, ItemType:string}} postItem
 */
export function updateMod(postItem) {
    doModsRequestAndChangeInStore("update", postItem);
}

export function addSuits(postItems) {
    doSuitsRequestAndChangeInStore("add", postItems);
}

export function removeSuits(postItems) {
    doSuitsRequestAndChangeInStore("remove", postItems);
}

export function updateSuit(postItems) {
    doSuitsRequestAndChangeInStore("update", postItems);
}

async function doModsRequestAndChangeInStore(action, body) {
    const user = get(userStore);
    const response = await fetch(`${serverUrl}/mods/${action}/${user.id}`, {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify(body),
    });
    const mods = await response.json();
    const { RawUpgrades, Upgrades } = mods;
    user.inventory.RawUpgrades = RawUpgrades;
    user.inventory.Upgrades = Upgrades;
    userStore.set(user);
}

async function doSuitsRequestAndChangeInStore(action, body) {
    const user = get(userStore);
    const response = await fetch(`${serverUrl}/suits/${action}/${user.id}`, {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify(body),
    });
    const { Suits } = await response.json();
    user.inventory.Suits = Suits;
    userStore.set(user);
}
