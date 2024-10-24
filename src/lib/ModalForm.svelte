<script lang="ts" context="module">
    export type ModalFieldDefinition = {
        name: string,
        type: 'text' | 'dropdown' | 'file' | 'checkbox',
        required: boolean,
        options?: string[],
    };
    export type FilledModalFields = { [key: string]: string | number | boolean | null };
    export type ModalAction = { name: string, callback: (values: FilledModalFields) => Promise<void> };

    export function newTextField(name: string, required: boolean): ModalFieldDefinition {
        return {
            name,
            type: "text",
            required,
        };
    }

    export function newDropdownField(name: string, required: boolean, options: string[]): ModalFieldDefinition {
        return {
            name,
            type: "dropdown",
            required,
            options,
        };
    }

    export function newFileField(name: string, required: boolean): ModalFieldDefinition {
        return {
            name,
            type: "file",
            required,
        };
    }
</script>

<script lang="ts">
    import { openDialog } from '$lib/util';
    import { toRelativePath } from './api';
    import Modal from './Modal.svelte';

    export let title = '';
    export let fields: ModalFieldDefinition[] = [];
    export let actions: ModalAction[] = [];

    const bindings = fields.reduce((acc, field) => {
        if (field.type === 'checkbox') {
            acc[field.name] = false;
        } else if (field.type === 'dropdown' || field.type === 'file') {
            acc[field.name] = null;
        } else {
            acc[field.name] = '';
        }
        return acc;
    }, {} as { [key: string]: string | number | boolean | null });

    let modal: Modal;
    let errorMessage: string | undefined = undefined;

    export const showModal = () => {
        modal.showModal();
    };

    export const hideModal = () => {
        modal.hideModal();
    };

    export const fillFields = (values: FilledModalFields) => {
        clearFields();
        fields.forEach(field => {
            bindings[field.name] = values[field.name];
        });
    };

    export const clearFields = () => {
        fields.forEach(field => {
            if (field.type === 'checkbox') {
                bindings[field.name] = false;
            } else if (field.type === 'dropdown' || field.type === 'file') {
                bindings[field.name] = null;
            } else {
                bindings[field.name] = '';
            }
        });
    };

    async function runCallbackWithFormData(callback: (values: FilledModalFields) => Promise<void>) {
        const missingFields: string[] = [];
        fields.forEach((field) => {
            if (field.required) {
                if ((field.type === 'text' && bindings[field.name] === '') ||
                    (field.type === 'file' && bindings[field.name] === null) ||
                    (field.type === 'dropdown' && bindings[field.name] === null)) {
                        missingFields.push(field.name);
                }
            }
        });
        if (missingFields.length > 0) {
            errorMessage = `Missing required fields: ${missingFields.join(', ')}`;
            return;
        }

        const formData: FilledModalFields = {};
        fields.forEach((field) => {
            if (field.type === 'text' && bindings[field.name] === '') {
                formData[field.name] = null;
            } else {
                formData[field.name] = bindings[field.name];
            }
        });
        await callback(formData);
        hideModal();
    }
</script>

<Modal bind:this={modal} bind:title>
    <div class="form">
        {#each fields as field}
            <label for={field.name}>{field.name}</label>
            {#if field.type === 'text'}
                <input type="text" id={field.name} name={field.name} bind:value={bindings[field.name]} />
            {:else if field.type === 'dropdown' && field.options}
                <select size="1" id={field.name} name={field.name} bind:value={bindings[field.name]}>
                    <option value={null}>(none)</option>
                    {#each field.options as option, i}
                        <option value={i}>{option}</option>
                    {/each}
                </select>
            {:else if field.type === 'file'}
                <span>
                    <button on:click={async () => {
                        const path = await openDialog();
                        if (!path) return;
                        const relativePath = await toRelativePath(path);
                        if (!relativePath) {
                            errorMessage = "File must be in project's directory."
                        }
                        bindings[field.name] = relativePath;
                    }}>Choose File</button>
                    {#if bindings[field.name]}
                        <em>{bindings[field.name]}</em>
                    {:else}
                        <em>No file selected.</em>
                    {/if}
                </span>
            {:else if field.type === 'checkbox'}
                // why is this not type checking????????
                <input type="checkbox" id={field.name} name={field.name} bind:checked={bindings[field.name]} />
            {/if}
        {/each}

        <br>
        
        {#if errorMessage}
            <p class="error-message">{errorMessage}</p>
        {/if}
        
        {#each actions as action}
            <button class="button-medium" on:click={() => runCallbackWithFormData(action.callback)}>{action.name}</button>
        {/each}
    </div>
</Modal>

<style>
    div.form {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        width: 30rem;
        margin: 1rem auto;
    }

    label {
        font-size: 1rem;
        margin-top: 1rem;
        font-weight: bold;
        text-transform: capitalize;
    }

    input {
        font-size: 1.25rem;
        padding: 0.5rem;
        width: 100%;
    }

    select {
        font-size: 1.25rem;
        padding: 0.5rem;
        width: 100%;
    }

    .button-medium:disabled {
        cursor: not-allowed;
    }

    .error-message {
        color: red;
        font-size: 0.9rem;
        margin-top: 0.5rem;
    }
</style>
