export function filterBySearch(searchTerm, array, keysArr) {
    return array.filter((item) => {
        return keysArr.find((key) => item[key]?.toLowerCase().includes(searchTerm.toLowerCase()));
    });
}

export function sortBy(array, key, direction = "asc") {
    return array.slice().sort((a, b) => (direction === "asc" ? a[key] - b[key] : b[key] - a[key]));
}
