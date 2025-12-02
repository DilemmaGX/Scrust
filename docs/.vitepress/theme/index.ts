import DefaultTheme from 'vitepress/theme'
import { onMounted, watch, nextTick } from 'vue'
import { useRoute } from 'vitepress'
import './style.css'

export default {
  extends: DefaultTheme,
  setup() {
    const route = useRoute()
    
    const renderBlocks = async () => {
      if (typeof window === 'undefined') return;
      
      try {
        // Dynamic import to avoid SSR issues and handle export type
        const module = await import('scratchblocks');
        const scratchblocks = module.default || module;
        
        if (scratchblocks && scratchblocks.renderMatching) {
           scratchblocks.renderMatching('pre.blocks', {
            style: 'scratch3',
            languages: ['en']
          })
          scratchblocks.renderMatching('code.b', {
            inline: true,
            style: 'scratch3',
            scale: 0.8
          })
        }
      } catch (e) {
        console.error('Failed to load scratchblocks', e);
      }
    }

    onMounted(() => {
      renderBlocks()
    })

    watch(
      () => route.path,
      () => nextTick(() => renderBlocks())
    )
  }
}
