import { JSXElement } from "solid-js";
import { DensityCoefficient, Differences, TankProfile, Tankers, Tanks, Vessels } from "../components";

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
                    "density_coefficient",
                    "difference_type",
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
                    "fuel_type",
                    "tank_type",
                    "full_volume",
                    "safe_volume",
                    "previous_volume",
                    "current_volume",
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

// export type {ItemContainer};
export default NavigationItems;