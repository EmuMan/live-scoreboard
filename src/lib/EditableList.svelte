<script lang="ts">
    import ModalForm, { type FilledModalFields, type ModalFieldDefinition } from "./ModalForm.svelte";

    type T = $$Generic;

    export let items: T[] = [];
    export let itemTemplate: (item: T) => Promise<string>;
    
    export let selectedItem: T | undefined = undefined;
    export let selectedIndex: number | undefined = undefined;
    export let onSelect: (item: T | undefined) => void = () => {};
    export let onUpdate: (items: T[], from?: number, to?: number) => void;
    export let fields: ModalFieldDefinition[] = [];
    export let toFilledFields: (item: T) => FilledModalFields;
    export let fromFilledFields: (fields: FilledModalFields, oldItem?: T) => T;

    export let width = "100%";
    export let height = "100%";

    let modal: ModalForm;
    let isEditing = false;

    $: onSelect(selectedItem);

    function deleteListItem(index: number) {
        items = items.filter((_, i) => i !== index);
        if (items.length === 0) {
            selectedIndex = undefined;
            selectedItem = undefined;
        } else {
            selectedIndex = selectedIndex === undefined ? items.length - 1 :
                Math.min(selectedIndex, items.length - 1);
            selectedItem = items[selectedIndex];
        }
        onUpdate(items, index);
    }

    function moveListItem(index: number, displacement: number) {
        if (displacement === 0) return;
        if (index < 0 || index >= items.length) return;
        let newIndex = index + displacement;
        if (newIndex < 0) {
            newIndex = 0;
        }
        if (newIndex >= items.length) {
            newIndex = items.length - 1;
        }
        if (newIndex === index) return;
        const item = items[index];
        const without = items.filter((_, i) => i !== index);
        items = [...without.slice(0, newIndex), item, ...without.slice(newIndex)];
        selectedIndex = newIndex;
        selectedItem = items[selectedIndex];
        onUpdate(items, index, newIndex);
    }
</script>

<div class="container">
    <div class="list" style="width: {width}; height: {height};">
        {#each items as item, i}
            {#await itemTemplate(item)}
                <button>Loading...</button>
            {:then itemTemplate}
                <button class:active={i === selectedIndex} on:click={() => {
                    selectedItem = item;
                    selectedIndex = i;
                }}>{@html itemTemplate}</button>
            {/await}
        {/each}
    </div>
    <div class="buttons">
        <button on:click={() => {
            isEditing = false;
            modal.clearFields();
            modal.showModal();
        }}>
            <img class="icon" src="/icons/icons8-plus.svg" alt="Add">
        </button>
        <button on:click={() => {
            if (selectedItem !== undefined) {
                isEditing = true;
                modal.fillFields(toFilledFields(selectedItem));
                modal.showModal();
            }
        }}>
            <img class="icon" src="/icons/icons8-edit.svg" alt="Edit">
        </button>
        <button on:click={() => {
            if (selectedIndex !== undefined) {
                deleteListItem(selectedIndex);
            }
        }}>
            <img class="icon" src="/icons/icons8-delete.svg" alt="Delete">
        </button>
        <button on:click={() => {
            if (selectedIndex !== undefined) {
                moveListItem(selectedIndex, -1);
            }
        }}>
            <img class="icon" src="/icons/icons8-collapse-arrow.svg" alt="Move Up">
        </button>
        <button on:click={() => {
            if (selectedIndex !== undefined) {
                moveListItem(selectedIndex, 1);
            }
        }}>
            <img class="icon" src="/icons/icons8-expand.svg" alt="Move Down">
        </button>
    </div>
</div>

<ModalForm
    bind:this={modal}
    title={isEditing ? "Edit" : "Add"}
    fields={fields}
    actions={[
        {
            name: 'Submit',
            callback: async (values) => {
                const newItem = fromFilledFields(values, isEditing ? selectedItem : undefined);
                if (isEditing && selectedIndex !== undefined) {
                    items = items.map((v, i) => i === selectedIndex ? newItem : v)
                    selectedItem = newItem;
                } else {
                    items = items.concat([newItem]);
                    selectedIndex = items.length - 1;
                    selectedItem = newItem;
                }
                onUpdate(items);
            }
        }
    ]} />

<style>
    .container {
        display: flex;
        flex-direction: row;
    }

    .list {
        display: flex;
        flex-direction: column;
        overflow-y: auto;
        border: 1px solid #666;
        border-radius: 0.5rem;
    }
    
    button {
        padding: 0.5rem;
        margin: 0;
        border: none;
        background-color: transparent;
        cursor: pointer;
        text-align: left;
    }

    button.active {
        background-color: #333;
    }

    .buttons {
        display: flex;
        flex-direction: column;
    }

    .buttons button {
        padding: 0.2rem 0.5rem;
    }

    .icon {
        width: 1.5rem;
        height: 1.5rem;
        object-fit: contain;
    }
</style>
