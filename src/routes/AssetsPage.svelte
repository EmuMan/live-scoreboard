<script lang="ts">
    import { convertFileSrc } from "@tauri-apps/api/core";
    import type { Asset } from '$lib/models';
    import EditableList from '$lib/EditableList.svelte';
    import Container from '$lib/Container.svelte';
    import Section from '$lib/Section.svelte';
    import * as api from "$lib/api";
    import { newFileField, newTextField, type FilledModalFields } from "$lib/ModalForm.svelte";

    export let assets: Asset[];

    function assetFromFilledFields(fields: FilledModalFields, oldItem?: Asset): Asset {
        return {
            name: fields["Name"] as string,
            path: fields["Path"] as string,
        };
    }
</script>

<Container>
    <h1>Assets</h1>
    <Section>
        <p>Assets</p>
        <EditableList
            items={assets}
            itemTemplate={async (asset) => {
                const absolutePath = await api.fromRelativePath(asset.path);
                if (absolutePath === null) {
                    return `${asset.name} <span style="color: red;">File not found</span>`;
                }
                const path = convertFileSrc(absolutePath);
                return `${asset.name} <img src=${path} alt=${asset.name} style="max-width: 3rem; max-height: 3rem;" />`;
            }}
            onUpdate={(items) => { assets = items; }}
            fields={[
                newTextField("Name", true),
                newFileField("Path", true),
            ]}
            toFilledFields={(item) => {
                return {
                    "Name": item.name,
                    "Path": item.path,
                };
            }}
            fromFilledFields={assetFromFilledFields}
            height={"10rem"} />
    </Section>
</Container>

<style>
    p {
        margin-top: 0;
    }
</style>
