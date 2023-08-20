import { createResource, createSignal } from "solid-js";
import Title from "../title";
import Table from "../table/table";
import DropDownMenu from "../dropDownMenu/dropDownMenu";
import { invoke } from "@tauri-apps/api";
import { NavigationItems } from "../../data";


const getDifferences = async (vesselId: string)=> {
    if (vesselId){
        return await invoke("get_daily_differences_for_current_month", {"vesselId":vesselId} )
    }
};
const getVessels = async () => {
    return await invoke("get_vessels");
};


export default function Dashboard(){
    const [activeVesselId, setActiveVesselId] = createSignal("");
    const [differences, {refetch}] = createResource(activeVesselId, getDifferences)
    const [vessels] = createResource(getVessels);
    const handleChangeVesselId = (id: string) => {
        setActiveVesselId(id);
    };


    return (
        <div class="screen-container">
            <DropDownMenu 
                items={vessels()}
                displayValueKey="name"
                identifyValueKey="id"
                setSignalCallback={handleChangeVesselId}
                placeholder="Select vessel..."
            />
            <Title value="Dashboard"/>
            <Table 
                records={differences()} 
                headers={NavigationItems().filter(
                    item => item.name === "dashboard"
                )[0].item.tableHeaders}
            />
        </div>
    )
}