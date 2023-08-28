import { Show, createResource, createSignal } from "solid-js"
import Title from "../title"
import Table from "../table/table"
import { invoke } from "@tauri-apps/api"
import { createStore } from "solid-js/store"
import {
    AddRecordModal,
    UploadFileModal,
} from "../modals"
import DropDownMenu from "../dropDownMenu/dropDownMenu"
import { InputField } from "../inputField"
import { NavigationItems } from "../../data"


const getVessels = async () => {
    return await invoke("get_vessels");
};
const addTankProfile = async (form: Object) => {
    await invoke("add_tank_profile", {tankProfile: form})
};

export default function HeightCorrections(){
    const [activeVessel, setActiveVessel] = createSignal("");
    const [activeTank, setActiveTank] = createSignal("")
    const getTanks = async (id: string) => {
        if (activeVessel()){
            return await invoke("get_tanks", {"vesselId": id});
        }    
    };
    const getTankCorrections = async (tankId: string)=> {
        if (activeVessel() && activeTank()){
            return await invoke("get_tank_corrections", {"tankId": tankId})
        }
    };
    const [tanks] = createResource(activeVessel, getTanks);
    const [vessels] = createResource(getVessels);
    const [tankCorrections, {refetch}] = createResource(activeTank, getTankCorrections)
    const [form, setForm] = createStore(
        {
            tank_id: "",
            vessel_id: "",
            tank_trim: 0.0,
            height_correction: 0.0,
        }
    );
    const [uploadForm, setUploadForm] = createStore(
        {
            tankId: "",
            filePath: "",
        }
    );
    const submitForm = async () => {
        await addTankProfile(form);
        refetch();   
    };
    const updateUploadForm = async (event: Event) => {
        const inputElement = event.currentTarget as HTMLInputElement;
        let file: File = inputElement.files[0]
        setUploadForm(
            {
                tankId: activeTank(),
                filePath: `/home/nikita/Documents/${file.name}`
            }
        );
    };
    const submitUploadForm = async () => {
        await invoke("add_tank_profiles", uploadForm);
        refetch();   
    }
    const handleChangeVessel = (id: string) => {
        setActiveVessel(id);
        setForm({vessel_id: activeVessel()});
    };
    const handleChangeTank = (id: string) => {
        setActiveTank(id);
        setForm({tank_id: activeTank()});
    };
    const updateFormField = (fieldName: string) => (event: Event) => {
        const inputElement = event.currentTarget as HTMLInputElement;
        setForm({
            [fieldName]: inputElement.value
        });
    };    

    return (
        <div class="screen-container">
            <Title value="Height corrections"/>
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
            <Table 
                records={tankCorrections()} 
                headers={NavigationItems().filter(item => item.name === "corrections")[0].item.tableHeaders}
            />
            <AddRecordModal 
                buttonText="Add height correction"
                title="Add height correction"
                add_record_callback={submitForm}
            >
                <InputField
                    placeholder="Tank trim" 
                    type="number" 
                    name="tank_trim"
                    id="tank_trim" 
                    onChange={updateFormField("tank_trim")} 
                />
                <InputField
                    placeholder="Height correction" 
                    type="number" 
                    name="height_correction"
                    id="height_correction" 
                    onChange={updateFormField("height_correction")} 
                />
            </AddRecordModal>
            <UploadFileModal 
                buttonText="Upload corrections from csv" 
                title="Upload tank corrections" 
                submitFormCallback={submitUploadForm}
            >
                <input type="file" onChange={updateUploadForm} id="tankCorrectionsFile" name="filename"/>
            </UploadFileModal>
        </div>
    )
}