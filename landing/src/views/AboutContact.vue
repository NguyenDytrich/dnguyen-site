<template>
  <div>
    <About />
    <Contact />
  </div>
</template>

<script lang="ts">
import { Component, Vue } from "vue-property-decorator";
import About from "./About.vue";
import Contact from "./Contact.vue";

@Component({
  components: {
    About,
    Contact
  }
})
export default class AboutContact extends Vue {}
</script>

<style scoped lang="scss">
div {
  padding-top: 5rem;
}
.content-container {
  margin-bottom: 2rem;
}
</style>
