import { createResource } from "solid-js"
import Title from "../title"
import Table from "../table/table"
import UploadFileModal from "../modals/uploadFileModal"
import { invoke } from "@tauri-apps/api/core"
import { createStore } from "solid-js/store"
import { AddRecordModal } from "../modals"
import { FactorType, NavigationItems } from "../../data"
import { InputField } from "../inputField"
import SearchForm from "../searchForm/searchForm"
import DropDownMenu from "../dropDownMenu/dropDownMenu"


export default function DensityCoefficient(){
    const addDensityCoefficient = async () => {
        if (form.temperature && form.coefficient && form.density) {
            await invoke("add_density_coefficient", {densityCoefficient: form});
        }
    };
    const [form, setForm] = createStore(
        {
            temperature: "",
            density: "",
            coefficient: 0.0,
            factor: "",
            pagination: {
                page: 1,
                per_page: 17,
                total_amount: 0
            }
        }
    );
    const getDensityCoefficientsAmount = async() => {
        return await invoke("get_density_coefficients_amount", {searchForm: form});
    };
    const getDensityCoefficients = async () => {
        const amount = await getDensityCoefficientsAmount()

        setForm(
            {
                pagination: {
                    ...form.pagination, total_amount: amount
                }
            }
        );
        return await invoke("get_density_coefficients", {searchForm: form});
    };
    const [densityCoefficients, {refetch}] = createResource(getDensityCoefficients);
    const [uploadForm, setUploadForm] = createStore(
        {
            filePath: "",
            factor: "",
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
        <div class="screen-container">
            <Title value="Density coefficients"/>
            <SearchForm 
                fields={
                    [
                        {name: "temperature", type: "number", step: "1", min: "10", max: "90"},
                        {name: "density", type: "number", step: "0.001", min: "0.7", max: "1"},
                    ]
                }
                formChangeCallback={updateFormField}
                submitCallback={submitSearchForm}
            />
            <Table
                records={densityCoefficients()}
                headers={
                    NavigationItems().filter(item => item.name === "densities")[0].item.tableHeaders
                }
                pagination={form.pagination}
                mutateSignal={setForm}
                submitFormCallback={submitSearchForm}
            />
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
                <DropDownMenu 
                    items={FactorType()}
                    displayValueKey="name"
                    identifyValueKey="id"
                    setSignalCallback={
                        (factor: string)=>{setForm({factor: factor})}
                    }
                    placeholder="Select factor..."
                />
            </AddRecordModal>
            <UploadFileModal 
                buttonText="Upload coefficients from csv" 
                title="Upload density coefficients" 
                submitFormCallback={submitUploadForm}
            >
                <input 
                    type="file"
                    onChange={updateUploadForm}
                    id="densityCoefficientsFile" 
                    name="filename"
                />
                <DropDownMenu 
                    items={FactorType()}
                    displayValueKey="name"
                    identifyValueKey="id"
                    setSignalCallback={
                        (factor: string)=>{setUploadForm({factor: factor})}
                    }
                    placeholder="Select factor..."
                />
            </UploadFileModal>
        </div>
    )
}