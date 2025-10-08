<script>
  import Navigation from '../components/Navigation.svelte';
</script>

<Navigation />
<main>
  <slot />
</main>

<style>
  main {
    min-height: 100vh;
  }
</style>