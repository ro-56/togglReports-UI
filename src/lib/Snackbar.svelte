<script  lang="ts">
    import IconButton from '@smui/icon-button';
    import Snackbar, { Label, Actions } from '@smui/snackbar';


    export let component: Snackbar;
    export let type: string = "success";
    export let text: string = "";
    export let duration: number = 4000;

    let norm_type: string;
    $: norm_type = 
        (type == "success" || "warning" || "error")
        ? "action-"+type : 
        "";

    export function open() {
        component.open();
    }
</script>

<Snackbar bind:this={component} class={norm_type} bind:timeoutMs={duration}>
    <Label>
        {text}
    </Label>
    <Actions>
        <IconButton class="material-icons" title="Dismiss">
            close
        </IconButton>
    </Actions>
</Snackbar>