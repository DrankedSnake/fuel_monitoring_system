import { invoke } from "@tauri-apps/api/core";
import { createResource, createSignal } from "solid-js";

import Title from "../../title";
import Table from "../../table/table";
import { createStore } from "solid-js/store";
import { AddRecordModal } from "../../modals";
import DropDownMenu from "../../dropDownMenu/dropDownMenu";
import { InputField } from "../../inputField";
import { FuelType, NavigationItems, TankType } from "../../../data";
import TanksChart from "./tanksChart";
import { Tank } from "../../../types";

const getVessels = async ()  => {
    return await invoke("get_vessels");
};

type Vessel = {
    id: string,
    name: string,
    year: number,
}

export default function Tanks(){
    const [activeVessel, setActiveVessel] = createSignal<string>("");
    const [activeWidget, setActiveWidget] = createSignal("chart")
    const getTanks = async (id: string) => {
        if (activeVessel()){
            return await invoke("get_tanks", {"vesselId": id});
        }    
    }
    const [vessels] = createResource<Array<Vessel>>(getVessels);
    const [tanks, {refetch}] = createResource<Array<Tank>>(activeVessel, getTanks);
 

    const addTank = async (tank: any) => {
        if (activeVessel().length != 0){
            await invoke("add_tank", {"tank": tank});
            refetch(activeVessel());
        }
    }
    const [form, setForm] = createStore(
        {
            "tank_name": "",
            "full_volume": 0.0,
            "current_volume": 0.0,
            "safe_volume": 0.0,
            "previous_volume": 0.0,
            "current_mass": 0.0,
            "previous_mass": 0.0,
            "vessel_id": "",
            "fuel_type": "",
            "tank_type": "",
        }
    );

    const updateFormField = (fieldName: string) => (event: Event) => {
        const inputElement = event.currentTarget as HTMLInputElement;
        setForm({
            [fieldName]: inputElement.value
        });
    };

    const submitForm = async () => {
        await addTank(form);
    };

    const handleChangeVessel = (id: string) => {
        setActiveVessel(id);
        setForm({vessel_id: activeVessel()});
    };
    return (
        <div class="screen-container">
            <Title value="Tanks"/>
            <DropDownMenu 
                items={vessels()}
                displayValueKey="name"
                identifyValueKey="id"
                setSignalCallback={handleChangeVessel}
                placeholder="Select vessel..."
            />
            <TanksChart tanks={tanks()}/>
            <Table records={tanks()} 
                headers={
                    NavigationItems()
                    .filter(item => item.name === "tanks")[0].item.tableHeaders
                }
            />
            <AddRecordModal buttonText="Add tank" title="Add tank" add_record_callback={submitForm}>
                <InputField
                    labelText="Tank name"
                    placeholder="Enter tank name..."
                    type="text" 
                    name="tank_name" 
                    id="tank_name" 
                    onChange={updateFormField("tank_name")}
                    required=""
                />
                <InputField
                    labelText="Full volume"
                    placeholder="Enter full volume..."
                    type="number" 
                    id="full_volume" 
                    name="full_volume" 
                    onChange={updateFormField("full_volume")}
                    min="0"
                    step="0.001"
                />
                <InputField
                    labelText="Current volume" 
                    placeholder="Enter current volume..."
                    type="number" 
                    id="current_volume" 
                    name="current_volume" 
                    onChange={updateFormField("current_volume")} 
                    min="0"
                    step="0.001"
                />
                <InputField
                    labelText="Safe volume" 
                    placeholder="Enter coefficient 0.8 or percents 80..."
                    type="number" 
                    id="safe_volume" 
                    name="safe_volume" 
                    onChange={updateFormField("safe_volume")} 
                    min="0"
                    step="0.1"
                    max="100"
                />
                <InputField
                    labelText="Previous volume" 
                    placeholder="Enter previous volume..."
                    type="number" 
                    id="previous_volume" 
                    name="previous_volume" 
                    onChange={updateFormField("previous_volume")}
                    min="0"
                    step="0.001"
                />
                <InputField
                    labelText="Current mass" 
                    placeholder="Enter current mass..."
                    type="number" 
                    id="current_mass" 
                    name="current_mass" 
                    onChange={updateFormField("current_mass")}
                    min="0"
                    step="0.001"
                />
                <InputField
                    labelText="Previous mass" 
                    placeholder="Enter previous mass..."
                    type="number" 
                    id="previous_mass" 
                    name="previous_mass" 
                    onChange={updateFormField("previous_mass")} 
                    min="0"
                    step="0.001"
                />
                <DropDownMenu
                    items={FuelType()}
                    displayValueKey="name"
                    identifyValueKey="id"
                    setSignalCallback={(fuelType: string)=>{setForm({fuel_type: fuelType})}}
                    placeholder="Select fuel type..."
                />
                <DropDownMenu
                    items={TankType()}
                    displayValueKey="name"
                    identifyValueKey="id"
                    setSignalCallback={(tankType: string)=>{setForm({tank_type: tankType})}}
                    placeholder="Select tank type..."
                />
            </AddRecordModal>
        </div>
    )
}