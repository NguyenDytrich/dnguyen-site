<template>
  <div class="landing">
    <div class="landing-container">
      <div class="header">
        <span class="underline-container"
          >Hi, I'm <span class="underline">Dytrich</span>.</span
        >
      </div>
      <div class="gradient">
        I <span class="reveal">code</span>, make
        <span class="reveal">music</span>, and take
        <span class="reveal">pictures</span>.
      </div>
      <div style="display: flex; justify-content: center">
        <div class="learn-more">
          <router-link to="/about">Learn more</router-link>
        </div>
        <div class="learn-more">
          <a href="https://blog.dytrich-nguyen">Blog</a>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { Vue } from "vue-property-decorator";
export default class Landing extends Vue {}
</script>
<style scoped lang="scss">
.landing {
  letter-spacing: 2px;
  font-size: 2rem;
}

.header {
  margin-bottom: 0.5em;
}

.landing-container {
  display: inline-block;
  padding: 2rem;
  background: white;
  text-align: left;

  position: relative;

  left: 50%;
  // For use with sidebar
  //transform: translateX(-50%) translateX(-11rem) translateY(25vh);

  // For no sidebar
  transform: translateX(-50%) translateY(24vh);

  // box-shadow: 0px 0px 20px rgba(0, 0, 0, 0.7);
}

.learn-more {
  text-align: center;
  font-size: 1.7rem;

  width: 30%;
  padding: 0.5rem 0.5rem;
  margin: 1em auto 0 auto;
  background: #3b4252;
  color: white;

  border: 2px solid #2c3e50;
  cursor: pointer;

  transition: opacity 0.3s ease-in-out;

  a {
    color: inherit;

    &:visited {
      color: inherit;
    }
  }

  &:hover {
    opacity: 0.8;
  }
}

a {
  text-decoration: none;

  &:visited {
    color: #2c3e50;
  }
}

.underline-container {
  position: relative;
}

.underline {
  transition: all 0.3s ease-in-out;
  &::after {
    z-index: -1;
    content: "";
    position: absolute;
    height: 4px;
    margin: 6px 0 0;

    transition: all 0.3s ease-in-out;
    background: #f27121;
    opacity: 0.5;
    border-radius: 10px;

    /* underline anim */
    top: 0.85em;
    left: 4.57em;
    width: 0;
    animation-name: slide;
    animation-duration: 4s;
    animation-iteration-count: infinite;

    /* one time
    top: 2em;
    left: 4.57em;
    animation: 0.7s slide 1.5s 1;
    animation-fill-mode: forwards;
    */
  }
}

.gradient {
  background: linear-gradient(
    135deg,
    #8a2387,
    #e94057,
    #f27121,
    #e94057,
    #8a2387
  );
  -webkit-background-clip: text;
  background-size: 200% auto;

  animation: gslide 20s infinite;

  .reveal {
    -webkit-text-fill-color: transparent;
  }
}

@keyframes slide {
  0% {
    width: 0;
    left: 3.75em;
  }
  25% {
    width: 3.825em;
    left: 3.75em;
  }
  50% {
    left: calc(3.75em + 3.825em);
    width: 0;
  }
}
/* one time
@keyframes slide {
  from {
    width: 0;
  }
  to {
    width: 3.65em;
  }
}
*/

@keyframes gslide {
  to {
    background-position: 200% center;
  }
}
</style>
