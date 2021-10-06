<template>
  <div id="greeting">
    <!--
    <div class="greeting">
      <div class="header">Hi, I'm Dytrich.</div>
      <div>I code, make music, and take pictures.</div>
    </div>
    -->
    <div id="links">
      <ul>
        <li><router-link to="/about">About</router-link></li>
        <li><router-link to="/music">Music</router-link></li>
        <li><router-link to="/code">Code</router-link></li>
        <li><router-link to="/photography">Photography</router-link></li>
        <li><router-link to="/contact">Contact Me</router-link></li>
      </ul>
    </div>
  </div>
</template>

<script lang="ts">
import { Vue } from "vue-property-decorator";
export default class SideNav extends Vue {}
</script>

<style scoped lang="scss">
#greeting {
  padding: 1em;
}

ul {
  list-style: none;
  padding: 0;

  li {
    margin-bottom: 0.5rem;
    transition: all 0.2s ease-in-out;
    height: 1.2em;

    &:hover {
      transform: translateX(5px);
    }

    a {
      color: #2c3e50;
      text-decoration: none;
      border-bottom: 1px dotted #2c3e50;
      //border-bottom: 1px dotted #2c3e50;
      outline: 0;

      transition: all 0.2s;

      &:hover {
        opacity: 0.5;
      }

      &.router-link-exact-active {
        border-bottom: 1px solid #c7b198;
        font-size: 1.1em;
      }
    }
  }
}
</style>
