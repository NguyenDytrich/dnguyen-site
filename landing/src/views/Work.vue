<template>
  <div class="work-container">
    <h1>Work</h1>
    <div class="selector">
      <router-link to="/work/code">Featured</router-link>
      <router-link to="/work/code">Code</router-link>
      <router-link to="/work/code">Music</router-link>
      <router-link to="/work/code">Photos</router-link>
    </div>
    <!--
    <router-view name="work-view"> </router-view>
    -->
  </div>
</template>

<script lang="ts">
import { Vue } from "vue-property-decorator";
export default class Work extends Vue {}
</script>

<style scoped lang="scss">
.work-container {
}

.selector {
  a {
    display: inline-block;
    margin: 0 2rem;
  }
}
</style>
