import { Component } from "solid-js";
import { Dashboard, DensityCoefficient, Differences, TankProfile, Tanks, Vessels } from "../components";


type Item = {
    title: string,
    path: string,
    component: Component,
    tableHeaders: Array<string>,
    icon: string,
};

type ItemContainer = {
    name: string,
    item: Item,
};

const NavigationItems = (): Array<ItemContainer> => {
    return [
        {
            name: "dashboard",
            item: {
                title: "Dashboard",
                path: "/dashboard",
                component: Dashboard,
                tableHeaders: ["hfo_volume", "hfo_mass", "mgo_volume", "mgo_mass", "date"],
                icon: "icons/dashboard.png"
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
                    "height",
                    "trim",
                    "temperature",
                    "density",
                    "date_created",
                ],
                icon: "icons/difference.png"
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
                ],
                icon: "icons/tank.png"
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
                icon: "icons/profile.png"
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
                    "year",
                    "dead_weight",
                    "imo",
                    "company",
                ],
                icon: "icons/vessel.png"
            }
        },
        {
            name: "densities",
            item: {
                title: "Densities",
                path: "/density_coefficients",
                component: DensityCoefficient,
                tableHeaders: ["temperature", "density", "coefficient"],
                icon: "icons/temperature.png"
            }
        },
    ];
};

export default NavigationItems;