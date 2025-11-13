import type { Duration, StateEnum } from './types'

export function forceNumeric(
  value: string | number | undefined,
): number | undefined {
  if (value === undefined) return undefined
  const num = typeof value === 'string' ? parseFloat(value) : value
  return isNaN(num) ? undefined : num
}

// A function to fetch a URL and handle redirects manually
export async function fetchOrRedirect(
  url: string,
  info: RequestInit,
): Promise<Response> {
  info.redirect = 'manual' // Ensure manual redirect handling
  const response = await fetch(url, info)

  // Check if it's a redirect (301, 302, 303, 307, 308)
  if (response.type === 'opaqueredirect') {
    // Browser hides Location header in opaque redirects for security.
    // To actually redirect the browser:
    window.location.href = url
    console.log('Redirecting to:', url)
    return Promise.reject(new Error('Redirecting...'))
  }

  if (response.status >= 300 && response.status < 400) {
    // If CORS allows, we can read the Location header:
    const location = response.headers.get('Location')
    if (location) {
      window.location.href = location
      console.log('Redirecting to:', location)
      return Promise.reject(new Error('Redirecting...'))
    }
  }

  console.log('Fetch response status:', response.status)
  // Otherwise, normal success — return the data
  return response
}

export function stateEmoji(state: StateEnum): string {
  switch (state) {
    case 'Good':
      return '✅'
    case 'Warning':
      return '⚠️'
    case 'Critical':
      return '❌'
    case 'Incoming':
      return '📦'
    default:
      return ''
  }
}

export function stateColor(state: StateEnum): string {
  switch (state) {
    case 'Good':
      return 'text-(--zaiko-text)'
    case 'Warning':
      return 'text-(--zaiko-warning-color)'
    case 'Critical':
      return 'text-(--zaiko-bad-color)'
    case 'Incoming':
      return 'text-(--zaiko-link-color)'
    default:
      return 'text-(--zaiko-text)'
  }
}

export function parseISODuration(duration: string): Duration {
  const regex =
    /^P(?:(\d+)Y)?(?:(\d+)M)?(?:(\d+)W)?(?:(\d+)D)?(?:T(?:(\d+)H)?(?:(\d+)M)?(?:(\d+(?:\.\d+)?)S)?)?$/

  const matches = duration.match(regex)
  if (!matches) {
    throw new Error('Invalid ISO 8601 duration format')
  }

  const [, years, months, weeks, days, hours, minutes, seconds] = matches

  return {
    ...(years ? { years: parseInt(years) } : {}),
    ...(months ? { months: parseInt(months) } : {}),
    ...(weeks ? { weeks: parseInt(weeks) } : {}),
    ...(days ? { days: parseInt(days) } : {}),
    ...(hours ? { hours: parseInt(hours) } : {}),
    ...(minutes ? { minutes: parseInt(minutes) } : {}),
    ...(seconds ? { seconds: parseFloat(seconds) } : {}),
  }
}

export function toISODuration(obj?: Duration): string | undefined {
  if (!obj) return undefined

  let datePart = ''
  let timePart = ''

  if (obj.years) datePart += `${obj.years}Y`
  if (obj.months) datePart += `${obj.months}M`
  if (obj.weeks) datePart += `${obj.weeks}W`
  if (obj.days) datePart += `${obj.days}D`

  if (obj.hours) timePart += `${obj.hours}H`
  if (obj.minutes) timePart += `${obj.minutes}M`
  if (obj.seconds) timePart += `${obj.seconds}S`

  if (!datePart && !timePart) {
    return 'P0D' // default zero duration
  }

  return `P${datePart}${timePart ? 'T' + timePart : ''}`
}

export function addDurationToDate(duration: Duration): Date {
  const now = new Date()

  if (duration.years) now.setFullYear(now.getFullYear() + duration.years)
  if (duration.months) now.setMonth(now.getMonth() + duration.months)
  if (duration.weeks) now.setDate(now.getDate() + duration.weeks * 7)
  if (duration.days) now.setDate(now.getDate() + duration.days)
  if (duration.hours) now.setHours(now.getHours() + duration.hours)
  if (duration.minutes) now.setMinutes(now.getMinutes() + duration.minutes)
  if (duration.seconds) now.setSeconds(now.getSeconds() + duration.seconds)

  return now
}

export function dateToDuration(from: Date, to: Date): Duration {
  let delta = Math.abs(to.getTime() - from.getTime()) / 1000

  const years = Math.floor(delta / (3600 * 24 * 365))
  delta -= years * 3600 * 24 * 365

  const months = Math.floor(delta / (3600 * 24 * 30))
  delta -= months * 3600 * 24 * 30

  const weeks = Math.floor(delta / (3600 * 24 * 7))
  delta -= weeks * 3600 * 24 * 7

  const days = Math.floor(delta / (3600 * 24))
  delta -= days * 3600 * 24

  const hours = Math.floor(delta / 3600)
  delta -= hours * 3600

  const minutes = Math.floor(delta / 60)
  delta -= minutes * 60

  const seconds = Math.floor(delta)

  const duration: Duration = {}
  if (years) duration.years = years
  if (months) duration.months = months
  if (weeks) duration.weeks = weeks
  if (days) duration.days = days
  if (hours) duration.hours = hours
  if (minutes) duration.minutes = minutes
  if (seconds) duration.seconds = seconds

  return duration
}

export function durationToReadableString(duration: Duration): string {
  const parts = []
  if (duration.years) parts.push(`${duration.years} år`)
  if (duration.months) parts.push(`${duration.months} mån`)
  if (duration.weeks) parts.push(`${duration.weeks} v`)
  if (duration.days) parts.push(`${duration.days} d`)
  if (duration.hours) parts.push(`${duration.hours} h`)
  if (duration.minutes) parts.push(`${duration.minutes} min`)
  if (duration.seconds) parts.push(`${duration.seconds} s`)

  return parts.join(' ')
}

export function parseDate(dateStr: string): string {
  const date = new Date(dateStr)
  return isNaN(date.getTime()) || date.getTime() === 0
    ? 'Aldrig'
    : date.toLocaleDateString('sv')
}
