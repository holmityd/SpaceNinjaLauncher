import Dashboard from "./routers/dashboard/Dashboard.svelte";
import Mods from "./routers/mods/Mods.svelte";
import ModsAdd from "./routers/mods/ModsAdd.svelte";
import Suits from "./routers/suits/Suits.svelte";
import SuitsAdd from "./routers/suits/SuitsAdd.svelte";
import Users from "./routers/users/Users.svelte";

const routers = [
    {
        path: "/",
        title: "",
        component: Users,
    },
    {
        path: "/dashboard",
        title: "Dashboard",
        component: Dashboard,
    },
    {
        path: "/dashboard/mods",
        title: "Mods",
        back: "/dashboard",
        component: Mods,
    },
    {
        path: "/dashboard/mods/add",
        title: "Mods - Add",
        back: "/dashboard/mods",
        component: ModsAdd,
    },
    {
        path: "/dashboard/suits",
        title: "Suits",
        back: "/dashboard",
        component: Suits,
    },
    {
        path: "/dashboard/suits/add",
        title: "Suits - Add",
        back: "/dashboard/suits",
        component: SuitsAdd,
    },
];

export default routers;
