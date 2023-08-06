import { Show, createResource } from "solid-js";
import Title from "../title";
import Table from "../table/table";
import { invoke } from "@tauri-apps/api";
import { createStore } from "solid-js/store";
import AddRecordModal from "../addRecordModal";


const getVessels = async ()=>{
    return await invoke("get_vessels")
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
                <Table records={vessels()}/>
            </Show>
            <AddRecordModal buttonText="Add vessel" title="Add vessel" add_record_callback={submitForm}>
                <div>
                    <label for="vessel_name">Vessel name</label>
                    <input type="text" name="vessel_name" id="vessel_name" onChange={updateFormField("vessel_name")}/>
                </div>
            </AddRecordModal>
        </div>
    )
}