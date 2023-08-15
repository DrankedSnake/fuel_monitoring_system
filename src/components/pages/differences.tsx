import { Show, createResource, createSignal } from "solid-js";
import Title from "../title";
import Table from "../table/table";
import { createStore } from "solid-js/store";
import DropDownMenu from "../dropDownMenu/dropDownMenu";
import { invoke } from "@tauri-apps/api";
import { NavigationItems } from "../../data";


const getDifferences = async ()=> {
    return []
};
const getVessels = async () => {
    return await invoke("get_vessels");
};


export default function Differences(){
    const [differences, {refetch}] = createResource(getDifferences)
    const [activeVessel, setActiveVessel] = createSignal("");
    const [activeTank, setActiveTank] = createSignal("")
    const getTanks = async (id: string) => {
        if (activeVessel()){
            return await invoke("get_tanks", {"vesselId": id});
        }    
    };
    const getTankProfiles = async (tankId: string)=> {
        if (activeVessel() && activeTank()){
            return await invoke("get_tank_profiles", {"tankId": tankId})
        }
    };
    const [tanks] = createResource(activeVessel, getTanks);
    const [vessels] = createResource(getVessels);
    const [form, setForm] = createStore(
        {
            tank_id: "",
            vessel_id: "",
            tank_height: 0.0,
            tank_volume: 0.0,
            tank_mass: 0.0,
            fuel_type: "",
            difference_type: "",
        }
    );
    const handleChangeVessel = (id: string) => {
        setActiveVessel(id);
        setForm({vessel_id: activeVessel()});
    };
    const handleChangeTank = (id: string) => {
        setActiveTank(id);
        setForm({tank_id: activeTank()});
    };
    return (
        <div class="screen-container">
            <DropDownMenu 
                items={vessels()}
                displayValueKey="name"
                identifyValueKey="id"
                setSignalCallback={handleChangeVessel}
                placeholder="Select vessel..."
            />
            <DropDownMenu 
                items={tanks()}
                displayValueKey="name"
                identifyValueKey="id"
                setSignalCallback={handleChangeTank}
                placeholder="Select tank..."
            />
            <Title value="Differences"/>
                <Table 
                    records={differences()} 
                    headers={NavigationItems().filter(
                        item => item.name === "differences"
                    )[0].item.tableHeaders}/>
            <div class="operations-bar">
                <button>Add difference</button>
            </div>
        </div>
    )
}