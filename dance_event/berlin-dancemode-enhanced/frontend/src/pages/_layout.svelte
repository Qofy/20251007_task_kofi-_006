<script>
  import NavigationBar from '../components/NavigationBar.svelte'
</script>

<NavigationBar />

<main class="main-content">
  <slot />
</main>

<style>
  .main-content {
    min-height: calc(100vh - 200px);
    padding: 2rem;
  }
</style>