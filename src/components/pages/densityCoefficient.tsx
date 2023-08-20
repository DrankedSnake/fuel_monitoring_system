import { Show, createResource } from "solid-js"
import Title from "../title"
import Table from "../table/table"
import UploadFileModal from "../modals/uploadFileModal"
import { invoke } from "@tauri-apps/api"
import { createStore } from "solid-js/store"
import { AddRecordModal } from "../modals"


export default function DensityCoefficient(){
    const getDensityCoefficients = async () => {
        return await invoke("get_density_coefficients");
    };
    const addDensityCoefficient = async () => {
        await invoke("add_density_coefficient");
    };
    const [densityCoefficients, {refetch}] = createResource(getDensityCoefficients);
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
        await addDensityCoefficient(form);
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
                <Table records={densityCoefficients()} headers={["temperature", "density", "coefficient"]}/>
            </Show>
            <AddRecordModal buttonText="Add density coefficient" title="Add density coefficient">
                <div>
                    <label for="value">Value</label>
                    <input type="text" name="value" id="value" />
                </div>
            </AddRecordModal>
            <UploadFileModal buttonText="Upload coefficients from csv" title="Upload density coefficients" submitFormCallback={submitUploadForm}>
                <input type="file" onChange={updateUploadForm} id="densityCoefficientsFile" name="filename"/>
            </UploadFileModal>
        </div>
    )
}