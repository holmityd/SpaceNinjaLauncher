import { get } from "svelte/store";
import { userStore } from "../store/User.store";

export async function getUsers() {
    const response = await fetch("http://localhost:53426/accounts");
    const data = await response.json();
    return data;
}

/**
 * Fetches user data from the server and sets it in the user store.
 *
 * @returns {Promise<void>} A promise that resolves when the data has been fetched and set.
 */
export async function fetchUserData(user) {
    const response = await fetch(`http://localhost:53426/inventory/${user.id}`);
    user.inventory = await response.json();
    userStore.set(user);
}

/**
 * Sends remove mods request and removes mods from the user store.
 *
 * @param {Array<{ItemType: string, ItemCount: number, UpgradeFingerprint: string, _id: {$oid: string}}>} postItems - The items to be removed.
 */
export async function removeMods(postItems) {
    const user = get(userStore);
    const response = await fetch(`http://localhost:53426/mods/remove/${user.id}`, {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify(postItems),
    });
    user.inventory = await response.json();
    userStore.set(user);
    // // TODO - send request
    // removeModsFromStore(postItems);
}

/**
 * Sends add mods request and adds mods to the user store.
 *
 * @param {Array<{ItemType: string, ItemCount?: number, UpgradeFingerprint?: string}>} postItems - The items to be added.
 */
export async function addMods(postItems) {
    const user = get(userStore);
    const response = await fetch(`http://localhost:53426/mods/add/${user.id}`, {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify(postItems),
    });
    user.inventory = await response.json();
    userStore.set(user);
    // // TODO - send request
    // addModsToStore(postItems);
}

export async function updateMod(postItem) {
    const user = get(userStore);
    const response = await fetch(`http://localhost:53426/mods/update/${user.id}`, {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify(postItem),
    });
    user.inventory = await response.json();
    userStore.set(user);
    // // TODO - send request
    // updateModInStore(postItem);
}
