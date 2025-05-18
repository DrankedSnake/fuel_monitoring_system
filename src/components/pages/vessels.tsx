import { Show, createResource } from "solid-js";
import Title from "../title";
import Table from "../table/table";
import { invoke } from "@tauri-apps/api/core";
import { createStore } from "solid-js/store";
import { AddRecordModal } from "../modals";
import { NavigationItems } from "../../data";
import { InputField } from "../inputField";


const getVessels = async ()=>{
    return await invoke("get_vessels");
};

export default function Vessels(){
    const [vessels, {mutate, refetch}] = createResource(getVessels);
    const addVessel = async (vessel: any) => {
        let newVessel = await invoke("add_vessel", {"vessel": vessel}).then().catch();
        refetch();
    }

    const [form, setForm] = createStore(
        {
            "vessel_name": "",
            "vessel_year": 0,
            "imo": "",
            "company": "",
            "vessel_dead_weight": 0,
        }
    );

    const updateFormField = (fieldName: string) => (event: Event) => {
        const inputElement = event.currentTarget as HTMLInputElement;
        setForm({
          [fieldName]: inputElement.value
        });
      };

    const submitForm = async () => {
        await addVessel(form);
    };

    return (
        <div class="screen-container">
            <Title value="Vessels"/>
            <Show when={vessels()} fallback={<p>Loading...</p>}>
                <Table records={vessels()} 
                    headers={
                        NavigationItems()
                        .filter(item => item.name === "vessels")[0].item.tableHeaders
                    }
                />
            </Show>
            <AddRecordModal buttonText="Add vessel" title="Add vessel" add_record_callback={submitForm}>
                <InputField
                    placeholder="Vessel name" 
                    type="text" 
                    name="vessel_name" 
                    id="vessel_name" 
                    onChange={updateFormField("vessel_name")} 
                />
                <InputField
                    placeholder="Vessel year" 
                    type="text" 
                    name="vessel_year" 
                    id="vessel_year" 
                    onChange={updateFormField("vessel_year")}
                />
                <InputField
                    placeholder="Vessel dead weight" 
                    type="text" 
                    name="vessel_dead_weight" 
                    id="vessel_dead_weight" 
                    onChange={updateFormField("vessel_dead_weight")}
                />
                <InputField
                    placeholder="Company" 
                    type="text" 
                    name="company" 
                    id="company" 
                    onChange={updateFormField("company")}
                />
                <InputField
                    placeholder="Imo" 
                    type="text" 
                    name="imo" 
                    id="imo" 
                    onChange={updateFormField("imo")}
                />
            </AddRecordModal>
        </div>
    )
}