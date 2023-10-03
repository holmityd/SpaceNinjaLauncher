import { get } from "svelte/store";
import { userStore } from "../store/User.store";

export async function getUsers() {
    const response = await fetch("http://localhost:53426/accounts");
    const data = await response.json();
    return data;
}

/**
 * @param {{id: string, email: string, display_name: string}} user - The user object.
 */
export async function fetchUserData(user) {
    const response = await fetch(`http://localhost:53426/inventory/${user.id}`);
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

async function doModsRequestAndChangeInStore(action, body) {
    const user = get(userStore);
    const response = await fetch(`http://localhost:53426/mods/${action}/${user.id}`, {
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
