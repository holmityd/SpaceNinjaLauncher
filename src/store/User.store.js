import { get, writable } from "svelte/store";
import { v4 } from "uuid";

export const modalPage = writable(false);

const localUserDataKey = "UserData";

export const userStore = writable(
    localStorage.getItem(localUserDataKey) && JSON.parse(localStorage.getItem(localUserDataKey)),
);

userStore.subscribe((user) => {
    if (user) {
        localStorage.setItem(localUserDataKey, JSON.stringify(user));
    }
});

/**
 * Removes mods from the user store.
 *
 * @param {Array<{ItemType: string, UpgradeFingerprint: string, ItemId: {$oid: string}}>} postItems - The items to be removed.
 */
export function removeModsFromStore(postItems) {
    const itemsSet = new Set(
        postItems.map(({ ItemType, UpgradeFingerprint, ItemId }) =>
            JSON.stringify({ ItemType, UpgradeFingerprint, ItemId }),
        ),
    );
    const $userStore = get(userStore);

    $userStore.inventory.RawUpgrades = $userStore.inventory.RawUpgrades.filter(
        ({ ItemType, UpgradeFingerprint, ItemId }) =>
            !itemsSet.has(JSON.stringify({ ItemType, UpgradeFingerprint, ItemId })),
    );
    $userStore.inventory.Upgrades = $userStore.inventory.Upgrades.filter(
        ({ ItemType, UpgradeFingerprint, ItemId }) =>
            !itemsSet.has(JSON.stringify({ ItemType, UpgradeFingerprint, ItemId })),
    );

    userStore.set($userStore);
}

/**
 * Add mods to the user store.
 *
 * @param {Array<{ItemType: string, ItemCount?: number, UpgradeFingerprint?: string}>} postItems - The items to be added.
 */
export function addModsToStore(postItems) {
    const $userStore = get(userStore);

    postItems.forEach(({ ItemType, ItemCount, UpgradeFingerprint }) => {
        if (!UpgradeFingerprint) {
            const itemExist = $userStore.inventory.RawUpgrades.find((i) => i.ItemType == ItemType);
            if (itemExist) {
                itemExist.ItemCount += ItemCount;
            } else {
                $userStore.inventory.RawUpgrades.push({ ItemType, ItemCount });
            }
        } else {
            $userStore.inventory.Upgrades.push({
                ItemType,
                UpgradeFingerprint,
                ItemId: { $oid: v4() },
            });
        }
    });

    userStore.set($userStore);
}

export function updateModInStore(postItem) {
    const { ItemId, ItemType, UpgradeFingerprint } = postItem;
    const $userStore = get(userStore);
    const lvl = UpgradeFingerprint ? JSON.parse(UpgradeFingerprint).lvl || 0 : 0;

    if (postItem.ItemId) {
        // in Upgrades
        const itemIndex = $userStore.inventory.Upgrades.findIndex(
            (i) => i.ItemId.$oid === ItemId.$oid,
        );
        if (lvl > 0) {
            // if level>0 update mod in Uprgrades
            $userStore.inventory.Upgrades[itemIndex].UpgradeFingerprint = UpgradeFingerprint;
        } else {
            // if level==0 remove mod from Uprgrades and add to RawUpgrades
            $userStore.inventory.Upgrades.splice(itemIndex, 1);

            const item = $userStore.inventory.RawUpgrades.find((i) => i.ItemType === ItemType);
            if (item) {
                item.ItemCount++;
            } else {
                $userStore.inventory.RawUpgrades.push({ ItemType, ItemCount: 1 });
            }
        }
    } else {
        // in RawUpgrades
        if (lvl > 0) {
            // if level changed remove mod from
            const itemIndex = $userStore.inventory.RawUpgrades.findIndex(
                (i) => i.ItemType === ItemType,
            );
            $userStore.inventory.RawUpgrades[itemIndex].ItemCount--;
            if ($userStore.inventory.RawUpgrades[itemIndex].ItemCount <= 0) {
                $userStore.inventory.RawUpgrades.splice(itemIndex, 1);
            }
            $userStore.inventory.Upgrades.push({
                ItemType,
                UpgradeFingerprint,
                ItemId: {
                    $oid: v4(),
                },
            });
        }
    }

    userStore.set($userStore);
}
