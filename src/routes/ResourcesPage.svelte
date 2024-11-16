<script lang="ts">
    import { convertFileSrc } from "@tauri-apps/api/core";
    import type { Resources, ResourcePair } from '$lib/models';
    import EditableList from '$lib/EditableList.svelte';
    import Container from '$lib/Container.svelte';
    import Section from '$lib/Section.svelte';
    import * as api from "$lib/api";
    import { newFileField, newTextField, type FilledModalFields } from "$lib/ModalForm.svelte";

    export let resources: Resources;

    function resourceFromFilledImageFields(fields: FilledModalFields, oldItem?: ResourcePair): ResourcePair {
        return {
            name: fields["Name"] as string,
            value: fields["Path"] as string,
        };
    }

    function resourceFromFilledStringFields(fields: FilledModalFields, oldItem?: ResourcePair): ResourcePair {
        return {
            name: fields["Name"] as string,
            value: fields["Value"] as string,
        };
    }
</script>

<Container>
    <h1>Resources</h1>

    <Section>
        <p>Images</p>
        <EditableList
            items={resources.images}
            itemTemplate={async (image) => {
                const absolutePath = await api.fromRelativePath(image.value);
                if (absolutePath === null) {
                    return `${image.name} <span style="color: red;">Image not found</span>`;
                }
                const path = convertFileSrc(absolutePath);
                return `${image.name} <img src=${path} alt=${image.name} style="max-width: 3rem; max-height: 3rem;" />`;
            }}
            onUpdate={(items) => { resources.images = items; }}
            fields={[
                newTextField("Name", true),
                newFileField("Path", true),
            ]}
            toFilledFields={(item) => {
                return {
                    "Name": item.name,
                    "Path": item.value,
                };
            }}
            fromFilledFields={resourceFromFilledImageFields}
            height={"10rem"} />
        
        <br>
        <p>Strings</p>
        <EditableList
            items={resources.strings}
            itemTemplate={async (string) => {
                return `${string.name} <span style="color: yellow;">${string.value}</span>`;
            }}
            onUpdate={(items) => { resources.strings = items; }}
            fields={[
                newTextField("Name", true),
                newTextField("Value", true),
            ]}
            toFilledFields={(item) => {
                return {
                    "Name": item.name,
                    "Value": item.value,
                };
            }}
            fromFilledFields={resourceFromFilledStringFields}
            height={"10rem"} />
    </Section>
</Container>

<style>
    p {
        margin-top: 0;
    }
</style>
