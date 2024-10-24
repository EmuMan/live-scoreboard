<script context="module" lang="ts">
    import type { ComponentType } from "svelte";

    export type SideNavButton = {
        name: string;
        icon: string;
    };
</script>

<script lang="ts">
    export let buttons: SideNavButton[];
    export let visibilities: boolean[];

    function makePageVisible(i: number) {
        visibilities = visibilities.map((_, j) => i === j);
    }
</script>

<div class="container">
    <div class="sidebar">
        {#each buttons as button, i}
            <button class="sidebar-item" class:active={visibilities[i]} on:click={() => makePageVisible(i)}>
                <img class="sidebar-item-icon" src={button.icon} alt={button.name}>
            </button>
        {/each}
    </div>
    <div class="main-content">
        <slot></slot>
    </div>
</div>

<style>
    .container {
        display: flex;
        height: 100vh;
        width: 100vw;
    }

    .sidebar {
        display: flex;
        flex-direction: column;
        background-color: #333;
        color: white;
        width: 5rem;
        height: 100%;
    }

    .sidebar-item {
        display: flex;
        justify-content: center;
        align-items: center;
        width: 5rem;
        height: 5rem;
        cursor: pointer;
        background-color: #333;
        border: none;
        border-radius: 0;
    }

    .sidebar-item:hover {
        background-color: #555;
    }

    .active {
        background-color: #555;
    }

    .sidebar-item-icon {
        width: 2.5rem;
        height: 2.5 rem;
    }

    .main-content {
        width: 100%;
        height: 100%;
        background-color: #444;
        color: white;
        overflow-y: auto;
    }
</style>
