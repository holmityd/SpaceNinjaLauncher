export function filterBySearch(searchTerm, array, keysArr) {
    return array.filter((item) => {
        return keysArr.find((key) => {
            if (key.includes(".")) {
                const nestedItem = key.split(".").reduce((acc, key) => {
                    if (!acc) return;
                    if (key === "all") {
                        return acc.join();
                    }
                    return acc[key];
                }, item);
                return nestedItem?.toLowerCase().includes(searchTerm.toLowerCase());
            }
            return item[key]?.toLowerCase().includes(searchTerm.toLowerCase());
        });
    });
}

export function sortBy(array, key, direction = "asc") {
    return array.slice().sort((a, b) => (direction === "asc" ? a[key] - b[key] : b[key] - a[key]));
}
