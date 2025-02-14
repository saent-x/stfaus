<script lang="ts">
  export let tab_content_1;  
  export let tab_content_2;   
</script>

<div class="tabs rounded-none tabs-box justify-center w-[100%] bg-white">
    <input type="radio" name="my_tabs_1" class="tab h-8 checked:!bg-gray-950 checked:!text-secondary-content" aria-label="new" checked={true} />
    <div class="tab-content w-full mt-5">
        <svelte:component this={tab_content_1}/>
    </div>

    <input type="radio" name="my_tabs_1" class="tab h-8 checked:!bg-gray-950 checked:!text-secondary-content" aria-label="saved" />
    <div class="tab-content w-full mt-5">
        <svelte:component this={tab_content_2}/>
    </div>
</div>