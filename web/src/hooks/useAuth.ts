import { useState, useCallback } from 'react'
import { client } from '../api/client'
import { SEND_OTP, VERIFY_OTP } from '../api/operations'
import type { AuthResult, VerifyResult } from '../api/types'

export function useAuth() {
  const [memberId, setMemberId] = useState<string | null>(
    () => localStorage.getItem('memberId')
  )
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const sendOtp = useCallback(async (phone: string) => {
    setLoading(true)
    setError(null)
    try {
      const data = await client.request<{ sendOtp: AuthResult }>(SEND_OTP, { phone })
      return data.sendOtp.success
    } catch (e: any) {
      setError(e.message || 'Failed to send OTP')
      return false
    } finally {
      setLoading(false)
    }
  }, [])

  const verifyOtp = useCallback(async (phone: string, code: string) => {
    setLoading(true)
    setError(null)
    try {
      const data = await client.request<{ verifyOtp: VerifyResult }>(VERIFY_OTP, { phone, code })
      const { memberId: id, token } = data.verifyOtp
      // Set token as cookie for subsequent requests
      document.cookie = `token=${token}; path=/; max-age=${24 * 3600}; SameSite=Lax`
      localStorage.setItem('memberId', id)
      setMemberId(id)
      return true
    } catch (e: any) {
      setError(e.message || 'Failed to verify OTP')
      return false
    } finally {
      setLoading(false)
    }
  }, [])

  const logout = useCallback(() => {
    document.cookie = 'token=; path=/; max-age=0'
    localStorage.removeItem('memberId')
    setMemberId(null)
  }, [])

  return {
    isAuthenticated: !!memberId,
    memberId,
    loading,
    error,
    sendOtp,
    verifyOtp,
    logout,
  }
}
