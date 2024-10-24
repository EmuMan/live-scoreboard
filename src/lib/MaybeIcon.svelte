<script lang="ts">
    import { getUrlFromRelativePath } from "./api";

    export let path: string | null;
    export let size: string = "1rem";
</script>

<div>
    {#if path}
        {#await getUrlFromRelativePath(path)}
            <div style="width: {size}; height: {size};">Loading icon...</div>
        {:then url}
            {#if url}
                <img src={url} alt="icon" style={`width: ${size}; height: ${size}; object-fit: contain;`} />
            {:else}
                <p style="color: red;">Icon not found</p>
            {/if}
        {/await}
    {/if}
</div>
