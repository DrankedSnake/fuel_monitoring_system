import { JSXElement } from "solid-js";
import { Dashboard, DensityCoefficient, Differences, TankProfile, Tankers, Tanks, Vessels } from "../components";

type ComponentCallback = (props: Object) => JSXElement;

type Item = {
    title: string,
    path: string,
    component: ComponentCallback,
    tableHeaders: Array<string>,
};

type ItemContainer = {
    name: string,
    item: Item,
};

const NavigationItems = () => {
    return [
        {
            name: "dashboard",
            item: {
                title: "Dashboard",
                path: "/dashboard",
                component: Dashboard,
                tableHeaders: ["volume", "mass", "date"]
            }
        },
        {
            name: "tankers",
            item: {
                title: "Tankers",
                path: "/tankers",
                component: Tankers,
                tableHeaders: [],
            }
        },
        {
            name: "differences",
            item: {
                title: "Differences",
                path: "/differences",
                component: Differences,
                tableHeaders: [
                    "volume",
                    "mass",
                    "date_created",
                ],
            }
        },
        {
            name: "tanks",
            item: {
                title: "Tanks",
                path: "/tanks",
                component: Tanks,
                tableHeaders: [
                    "name",
                    "current_mass",
                    "previous_mass",
                    "full_volume",
                    "safe_volume",
                    "current_volume",
                    "fuel_type",
                    "tank_type",
                    "previous_volume",
                    "change_24_volume",
                    "bunkering_volume",
                ]
            }
        },
        {
            name: "tank_profiles",
            item: {
                title: "Tank profiles",
                path: "/tank_profiles",
                component: TankProfile,
                tableHeaders: [
                    "height", 
                    "volume",
                    "trim",
                ],
            }
        },
        {
            name: "vessels",
            item: {
                title: "Vessels",
                path: "/vessels",
                component: Vessels,
                tableHeaders: [
                    "name", 
                    "year"
                ],
            }
        },
        {
            name: "densities",
            item: {
                title: "Densities",
                path: "/density_coefficients",
                component: DensityCoefficient,
                tableHeaders: ["height", "temperature", "density"],
            }
        },
    ];
};

export default NavigationItems;