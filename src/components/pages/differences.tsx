import { createResource, createSignal } from "solid-js";
import Title from "../title";
import Table from "../table/table";
import { createStore } from "solid-js/store";
import DropDownMenu from "../dropDownMenu/dropDownMenu";
import { invoke } from "@tauri-apps/api/core";
import { NavigationItems } from "../../data";
import { AddRecordModal } from "../modals";
import { InputField } from "../inputField";


export default function Differences(){
    
    const getDifferencesAmount = async() => {
        setForm(
            {
                tank_id: activeTankId()
            }
        );
        return await invoke("get_differences_amount", {searchForm: form});
    };
    const getDifferences = async ()=> {
        
        if (activeTankId()){
            const amount = await getDifferencesAmount()
            setForm(
                {
                    pagination: {
                        ...form.pagination, total_amount: amount
                    }
                }
            );
            return await invoke("get_differences", {searchForm: form} )
        }
    };
    const getVessels = async () => {
        return await invoke("get_vessels");
    };
    const [activeVesselId, setActiveVesselId] = createSignal("");
    const [activeTankId, setActiveTankId] = createSignal("")
    const [differences, {refetch}] = createResource(activeTankId, getDifferences)
    const getTanks = async (id: string) => {
        if (activeVesselId()){
            return await invoke("get_tanks", {"vesselId": id});
        }    
    };
    const isProfileExists = async () => {
        if (form.tank_height && form.tank_trim) {
            const profile = await invoke(
                "get_tank_profile", 
                {tankId: form.tank_id, height: form.tank_height, trim: form.tank_trim}
            )
            
            if (!profile){
                return `Profile with ${form.tank_height} height or ${form.tank_trim} trim was not found.`;
            } else {
                return ""
            }
        }
    }
    const isDensityCoefficientInVacuumExists =async () => {
        if (form.temperature && form.density_in_vacuum){
            const coefficient = await invoke(
                "get_density_coefficient_in_vacuum",
                {temperature: form.temperature, density: form.density_in_vacuum}
            )

            if (!coefficient) {
                return `Density coefficient with ${form.temperature} temperature or ${form.density_in_vacuum} density was not found.`;
            } else {
                return ""
            }
        }
    }
    const isDensityCoefficientInAirExists =async () => {
        if (form.density_in_air){
            const coefficient = await invoke(
                "get_density_coefficient_in_air",
                {temperature: "15.0", density: form.density_in_air}
            )

            if (!coefficient) {
                return `Density coefficient with 15 temperature or ${form.density_in_air} density was not found.`;
            } else {
                return ""
            }
        }
    }
    const [tanks] = createResource(activeVesselId, getTanks);
    const [vessels] = createResource(getVessels);
    const [form, setForm] = createStore(
        {
            tank_id: "",
            tank_height: 0.0,
            fuel_type: "",
            temperature: 0.0,
            tank_trim: 0.0,
            density_in_vacuum: 0.0,
            density_in_air: 0.0,
            pagination: {
                page: 1,
                per_page: 17,
                total_amount: 0,
            }
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
        setForm({tank_id: activeTankId()});
    };
    const addDifference = async () => {
        if (form.tank_id && form.tank_height && form.tank_trim && form.temperature && form.density_in_vacuum) {
            await invoke(
                "add_difference", 
                { 
                    form: {
                        tank_id: form.tank_id,
                        tank_height: form.tank_height,
                        tank_trim: form.tank_trim,
                        temperature: form.temperature,
                        density_in_air: form.density_in_air,
                        density_in_vacuum: form.density_in_vacuum
                    }
                }
            );
            refetch();
        }
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
                pagination={form.pagination}
                mutateSignal={setForm}
            />
            <AddRecordModal buttonText="Add difference" add_record_callback={addDifference}>
                <InputField
                    labelText="Height"
                    placeholder="Enter height..."
                    type="number" 
                    id="tank_height" 
                    name="tank_height"
                    min="0"
                    max="20"
                    step="0.001"
                    onChange={updateFormField("tank_height")}
                />
                <InputField
                    labelText="Trim"
                    placeholder="Enter trim..."
                    type="number" 
                    id="tank_trim" 
                    name="tank_trim"
                    min="-3"
                    max="3"
                    step="0.5"
                    required
                    onChange={updateFormField("tank_trim")} 
                    validator={isProfileExists}
                />
                <InputField
                    labelText="Temperature"
                    placeholder="Enter temperature..."
                    type="number" 
                    id="temperature" 
                    name="temperature"
                    min="10"
                    max="90"
                    step="1"
                    required
                    onChange={updateFormField("temperature")} 
                />
                <InputField
                    labelText="Density in vacuum"
                    placeholder="Enter density in vacuum..."
                    type="number" 
                    id="density_in_vacuum" 
                    name="density_in_vacuum"
                    min="0.7"
                    max="1"
                    step="0.00001"
                    required
                    onChange={updateFormField("density_in_vacuum")}
                    validator={isDensityCoefficientInVacuumExists}
                />
                <InputField
                    labelText="Density in air"
                    placeholder="Enter density in air..."
                    type="number" 
                    id="density_in_air" 
                    name="density_in_air"
                    min="0.7"
                    max="1"
                    step="0.00001"
                    required
                    onChange={updateFormField("density_in_air")}
                    validator={isDensityCoefficientInAirExists}
                />
            </AddRecordModal>
        </div>
    )
}