import {
    addModsToStore,
    removeModsFromStore,
    updateModInStore,
    userStore,
} from "../store/User.store";

/**
 * Fetches user data from the server and sets it in the user store.
 *
 * @param {string} id - The user ID.
 * @returns {Promise<void>} A promise that resolves when the data has been fetched and set.
 */
export async function fetchUserData(id) {
    try {
        const response = await fetch("/api/user.json");
        const data = await response.json();
        userStore.set(data);
    } catch (error) {
        console.error("Error fetching data:", error);
    }
}

/**
 * Sends remove mods request and removes mods from the user store.
 *
 * @param {Array<{ItemType: string, UpgradeFingerprint: string, ItemId: {$oid: string}}>} postItems - The items to be removed.
 */
export function removeMods(postItems) {
    // TODO - send request
    removeModsFromStore(postItems);
}

/**
 * Sends add mods request and adds mods to the user store.
 *
 * @param {Array<{ItemType: string, ItemCount?: number, UpgradeFingerprint?: string}>} postItems - The items to be added.
 */
export function addMods(postItems) {
    // TODO - send request
    addModsToStore(postItems);
}

export function updateMod(postItem) {
    // TODO - send request
    updateModInStore(postItem);
}
