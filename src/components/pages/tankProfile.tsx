import { createResource, createSignal } from "solid-js"
import Title from "../title"
import Table from "../table/table"
import { invoke } from "@tauri-apps/api/core"
import { createStore } from "solid-js/store"
import {
    AddRecordModal,
    UploadFileModal,
} from "../modals"
import DropDownMenu from "../dropDownMenu/dropDownMenu"
import { InputField } from "../inputField"
import { NavigationItems, Pagination } from "../../data"
import SearchForm from "../searchForm/searchForm"


const getVessels = async () => {
    return await invoke("get_vessels");
};
const addTankProfile = async (form: Object) => {
    await invoke("add_tank_profile", {tankProfile: form})
};

export default function TankProfile(){
    const [activeVessel, setActiveVessel] = createSignal("");
    const [activeTank, setActiveTank] = createSignal("")
    const getTanks = async (id: string) => {
        if (activeVessel()){
            return await invoke("get_tanks", {"vesselId": id});
        }    
    };
    const getTankProfiles = async ()=> {
        if (activeVessel() && activeTank()){
            setForm({tank_id: activeTank()})
            const amount = await getTankProfilesAmount()

            setForm({pagination: {
                ...form.pagination, total_amount: amount
            }})
            return await invoke("get_tank_profiles", {searchForm: form})
        }
    };
    const getTankProfilesAmount = async () => {
        if (activeVessel() && activeTank()){
            setForm({tank_id: activeTank()})

            return await invoke("get_tank_profiles_amount", {searchForm: form})
        }
    };

    const [tanks] = createResource(activeVessel, getTanks);
    const [vessels] = createResource(getVessels);
    const [tankProfiles, {refetch}] = createResource(activeTank, getTankProfiles)
    const [form, setForm] = createStore(
        {
            tank_id: "",
            vessel_id: "",
            height: "",
            volume: "",
            trim: "",
            pagination: Pagination
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
    const submitSearchForm = () => {
        refetch();
    };

    return (
        <div class="screen-container">
            <Title value="Tanks profiles"/>
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
            <SearchForm 
                fields={
                    [
                        {name: "height", type: "number", step: "0.001", min: "0"},
                        {name: "trim", type: "number", step: "0.05", min: "-10", max: "10"},
                    ]
                }
                formChangeCallback={updateFormField}
                submitCallback={submitSearchForm}
            />
            <Table 
                records={tankProfiles()} 
                headers={NavigationItems().filter(item => item.name === "tank_profiles")[0].item.tableHeaders}
                pagination={form.pagination}
                mutateSignal={setForm}
                submitFormCallback={submitSearchForm}
            />
            <AddRecordModal 
                buttonText="Add tank profile"
                title="Add tank profile"
                add_record_callback={submitForm}
            >
                <InputField
                    placeholder="Tank height" 
                    type="number" 
                    name="tank_height"
                    id="tank_height" 
                    onChange={updateFormField("height")} 
                />
                <InputField
                    placeholder="Tank volume" 
                    type="number" 
                    name="tank_volume" 
                    id="tank_volume" 
                    onChange={updateFormField("volume")} 
                />
                <InputField
                    placeholder="Tank trim" 
                    type="number" 
                    name="tank_trim" 
                    id="tank_trim" 
                    onChange={updateFormField("trim")}
                    min="-2.0"
                    max="2.0"
                />
            </AddRecordModal>
            <UploadFileModal 
                buttonText="Upload profiles from csv" 
                title="Upload tank profiles" 
                submitFormCallback={submitUploadForm}
            >
                <input type="file" onChange={updateUploadForm} id="tankProfilesFile" name="filename"/>
            </UploadFileModal>
        </div>
    )
}