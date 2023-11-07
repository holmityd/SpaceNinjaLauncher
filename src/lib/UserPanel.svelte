<script>
    import { navigate } from "svelte-routing";
    import { Avatar, Dropdown, DropdownItem } from "flowbite-svelte";
    import { fetchUserData } from "../services/user.service";
    import { userStore } from "../store/User.store";

    // dropdown
    let dropdown = false;
    function hideDropdown() {
        dropdown = false;
    }
    function handleClickOutside() {
        hideDropdown();
    }

    // buttons
    function dashboard() {
        navigate("/dashboard");
        hideDropdown();
    }
    function sync() {
        const { display_name, email, id } = $userStore;
        fetchUserData({
            display_name,
            email,
            id,
        });
        hideDropdown();
    }
    function signOut() {
        navigate("/");
        hideDropdown();
    }
</script>

<div class="relative h-10 w-10">
    <Avatar class="acs cursor-pointer" src="/avatar.png" dot={{ color: "green" }} />

    <Dropdown bind:open={dropdown} triggeredBy=".acs" class="top-100 right-0 mt-1.5">
        <div slot="header" class="px-4 py-2">
            <span class="block text-sm text-white">{$userStore.display_name}</span>
            <span class="block truncate text-sm font-medium">{$userStore.email}</span>
        </div>
        <DropdownItem on:click={dashboard}>Dashboard</DropdownItem>
        <DropdownItem on:click={sync}>Sync</DropdownItem>
        <DropdownItem slot="footer" on:click={signOut}>Sign out</DropdownItem>
    </Dropdown>
</div>
