<script lang="ts">
    import TopAppBar, {Row, Section,Title} from '@smui/top-app-bar';
    import IconButton from '@smui/icon-button';

</script>

<div class="navbar">
    <Row>
        <Section>
            <IconButton href="/" class="material-icons"  style="color:gray">
                home
            </IconButton>
        </Section>
  
        <Section align="end" toolbar>
            <IconButton href="/settings" class="material-icons" style="color:gray" aria-label="Settings">
                settings
            </IconButton>
        </Section>
    </Row>
</div>

<style>
    .navbar {
        position: fixed;
        top: 0;
        left: 0;
        width: 100%;
        height: 56px;
        background-color: #2f2f2f;
        display: flex;
        align-items: center;
        justify-content: space-between;
        padding: 0 0.2rem;
        box-sizing: border-box;
        z-index: 100;
    }

</style>