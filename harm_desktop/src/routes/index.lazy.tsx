import { createLazyFileRoute } from '@tanstack/react-router'
import { Shell } from '../components/shell';

export const Route = createLazyFileRoute('/')({
  component: RouteComponent,
})

function RouteComponent() {
  return <Shell>Hello "/"!</Shell>
}
