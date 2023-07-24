<script lang="ts">
  import ModeSelector from './lib/ModeSelector.svelte'
  import DateInput from './lib/DateInput.svelte'

  let selected_start_date = "";
  let selected_end_date = "";
  let selected_mode;

  
  let options = [{
      idx: 0,
      label: "Normal",
      command: "normal",
      requires_date: false
    },
    {
      idx: 1,
      label: "Hard",
      command: "hard",
      requires_date: false
    },
    {
      idx: 2,
      label: "Challenge",
      command: "challenge",
      requires_date: false
    },
    {
      idx: 3,
      label: "Custom",
      command: "custom",
      requires_date: true
    }];

</script>

<!-- SMUI Styles -->
<link
  rel="stylesheet"
  href="node_modules/svelte-material-ui/themes/svelte-dark.css"
/>
<!-- Material Icons, Roboto, and Roboto Mono fonts -->
<link
  href="https://fonts.googleapis.com/css2?family=Material+Icons&Roboto+Mono:ital@0;1&family=Roboto:ital,wght@0,100;0,300;0,400;0,500;0,700;0,900;1,100;1,300;1,400;1,500;1,700;1,900&display=swap"
  rel="stylesheet"
  media="screen and (prefers-color-scheme: dark)"
/>


<main>
  <div class="page">

    <h1>Welcome to Tauri!</h1>

    <div>
      <div class="row">
        <a href="https://track.toggl.com/" target="_blank">
          <img src="/toggl_track_pink.svg" class="logo toggl-pink" alt="Toggl Track Logo" />
        </a>
      </div>
    
      <p>
        Click on the Tauri, Vite, and Svelte logos to learn more. 
        {selected_mode?.label} 
      </p>
    </div>

    <div>
        <ModeSelector label="Mode" bind:selected_option={selected_mode} available_options={options}/>
      
      {#if selected_mode?.requires_date}
      <div class="row">
        <DateInput label="Start date" bind:date={selected_start_date}/>
        <DateInput label="End date" bind:date={selected_end_date}/>
      </div>
      {/if}
    </div>

  </div>
</main>

<style>
  :root {
    background-color: #2f2f2f;
  }

  main {
    margin: 48px auto 0;
    min-width: 256px;
    padding: 1rem;
    background-color: #2f2f2f;
  }

  .page {
    transition: 0.15s linear;
    margin: 0;
    padding-top: 10vh;
    display: flex;
    flex-direction: column;
    justify-content: center;
    text-align: center;
  }
  .row {
    display: flex;
    justify-content: center;
  }


  .logo {
    height: 6em;
    padding: 1.5em;
    will-change: filter;
    transition: 0.75s;
  }
  .logo.toggl-pink:hover {
    filter: drop-shadow(0 0 2em #E57CD8);
  }

  .logo.toggl-purple:hover {
    filter: drop-shadow(0 0 2em #2C1138);
  }
</style>