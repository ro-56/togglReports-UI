
<script lang="ts">
    import NavBar from '$lib/NavBar.svelte';

</script>

<style>
    :root {
      background-color: #2f2f2f;
    }
    main {
      margin: 0 auto 0;
      min-width: 256px;
      padding: 1rem;
      padding-top: 56px;
      background-color: #2f2f2f;
    }
</style>



<NavBar/>

<main>
    <slot />
</main>