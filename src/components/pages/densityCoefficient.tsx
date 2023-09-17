import { Show, createResource } from "solid-js"
import Title from "../title"
import Table from "../table/table"
import UploadFileModal from "../modals/uploadFileModal"
import { invoke } from "@tauri-apps/api"
import { createStore } from "solid-js/store"
import { AddRecordModal } from "../modals"
import { NavigationItems } from "../../data"
import { InputField } from "../inputField"


export default function DensityCoefficient(){
    const getDensityCoefficients = async () => {
        return await invoke("get_density_coefficients");
    };
    const addDensityCoefficient = async () => {
        if (form.temperature && form.coefficient && form.density) {
            await invoke("add_density_coefficient", {densityCoefficient: form});
        }
    };
    const [densityCoefficients, {refetch}] = createResource(getDensityCoefficients);
    const [form, setForm] = createStore(
        {
            temperature: 0.0,
            density: 0.0,
            coefficient: 0.0,
        }
    );
    const [uploadForm, setUploadForm] = createStore(
        {
            filePath: "",
        }
    );
    const submitUploadForm = async () => {
        await invoke("add_density_coefficients", uploadForm);
        refetch();
    };
    const submitForm = async () => {
        await addDensityCoefficient();
        refetch();
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
    const updateUploadForm = async (event: Event) => {
        const inputElement = event.currentTarget as HTMLInputElement;
        let file: File = inputElement.files[0]
        setUploadForm(
            {
                filePath: `/home/nikita/Documents/${file.name}`
            }
        );
    };
    return (
        // TODO: refactor component using latest changes in forms inputs buttons and navigation items
        <div class="screen-container">
            <Title value="Density coefficients"/>

            <Show when={densityCoefficients()} fallback={<p>Loading...</p>}>
                <Table 
                    records={densityCoefficients()} 
                    headers={NavigationItems().filter(item => item.name === "densities")[0].item.tableHeaders}
                />
            </Show>
            <AddRecordModal 
                buttonText="Add density coefficient" 
                title="Add density coefficient"
                add_record_callback={submitForm}
            >
                <InputField
                    placeholder="Enter temperature..." 
                    type="number" 
                    name="temperature"
                    id="temperature"
                    min="10"
                    max="90"
                    step="1"
                    require
                    onChange={updateFormField("temperature")} 
                />
                <InputField
                    placeholder="Enter density..." 
                    type="number" 
                    name="density"
                    id="density"
                    min="0.7"
                    max="1"
                    step="0.002"
                    require
                    onChange={updateFormField("density")} 
                />
                <InputField
                    placeholder="Enter coefficient..." 
                    type="number" 
                    name="coefficient"
                    id="coefficient"
                    min="0.3"
                    max="1.25"
                    step="0.0001"
                    require
                    onChange={updateFormField("coefficient")} 
                />
            </AddRecordModal>
            <UploadFileModal buttonText="Upload coefficients from csv" title="Upload density coefficients" submitFormCallback={submitUploadForm}>
                <input type="file" onChange={updateUploadForm} id="densityCoefficientsFile" name="filename"/>
            </UploadFileModal>
        </div>
    )
}