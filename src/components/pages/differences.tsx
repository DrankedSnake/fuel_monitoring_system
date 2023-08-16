import { createResource, createSignal } from "solid-js";
import Title from "../title";
import Table from "../table/table";
import { createStore } from "solid-js/store";
import DropDownMenu from "../dropDownMenu/dropDownMenu";
import { invoke } from "@tauri-apps/api";
import { NavigationItems } from "../../data";
import { AddRecordModal } from "../modals";
import { InputField } from "../inputField";


const getDifferences = async (tankId: string)=> {
    if (tankId){
        return await invoke("get_differences", {"tankId":tankId} )
    }
};
const getVessels = async () => {
    return await invoke("get_vessels");
};


export default function Differences(){
    const [activeVesselId, setActiveVesselId] = createSignal("");
    const [activeTankId, setActiveTankId] = createSignal("")
    const [differences, {refetch}] = createResource(activeTankId, getDifferences)
    const getTanks = async (id: string) => {
        if (activeVesselId()){
            return await invoke("get_tanks", {"vesselId": id});
        }    
    };

    const [tanks] = createResource(activeVesselId, getTanks);
    const [vessels] = createResource(getVessels);


    const [form, setForm] = createStore(
        {
            tankId: "",
            tankHeight: 0.0,
            fuelType: "",
            differenceType: "",
            temperature: 0.0,
            tankTrim: 0.0,
            density: 0.0,
        }
    );

    const updateFormField = (fieldName: string) => (event: Event) => {
        const inputElement = event.currentTarget as HTMLInputElement;
        setForm({
            [fieldName]: inputElement.value
        });
    };

    const handleChangeVesselId = (id: string) => {
        setActiveVesselId(id);
    };
    const handleChangeTankId = (id: string) => {
        setActiveTankId(id);
        setForm({tankId: activeTankId()});
    };
    const addDifference = async () => {
        await invoke("add_difference",{difference: form});
        refetch();
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
            <DropDownMenu 
                items={tanks()}
                displayValueKey="name"
                identifyValueKey="id"
                setSignalCallback={handleChangeTankId}
                placeholder="Select tank..."
            />
            <Title value="Differences"/>
            <Table 
                records={differences()} 
                headers={NavigationItems().filter(
                    item => item.name === "differences"
                )[0].item.tableHeaders}
            />
            <AddRecordModal buttonText="Add difference" add_record_callback={addDifference}>
                <InputField
                    labelText="Height"
                    placeholder="Enter height..."
                    type="number" 
                    id="tank_height" 
                    name="tank_height" 
                    onChange={updateFormField("tank_height")} 
                />
                <InputField
                    labelText="Trim"
                    placeholder="Enter trim..."
                    type="number" 
                    id="tank_trim" 
                    name="tank_trim" 
                    onChange={updateFormField("tank_trim")} 
                />
                <InputField
                    labelText="Temperature"
                    placeholder="Enter temperature..."
                    type="number" 
                    id="temperature" 
                    name="temperature" 
                    onChange={updateFormField("temperature")} 
                />
                <InputField
                    labelText="Density"
                    placeholder="Enter density..."
                    type="number" 
                    id="density" 
                    name="density" 
                    onChange={updateFormField("density")} 
                />
            </AddRecordModal>
        </div>
    )
}